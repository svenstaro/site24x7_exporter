//! Module containing functions related to handling metrics.
use log::debug;

use crate::{
    site24x7_types::{self, CurrentStatusData},
    MONITOR_LATENCY_SECONDS_GAUGE, MONITOR_UP_GAUGE,
};

/// Set the Prometheus metrics for a specfic monitor.
fn set_metrics_for_monitors(monitors: &[site24x7_types::MonitorMaybe], monitor_group: &str) {
    for monitor_maybe in monitors {
        let monitor_type = monitor_maybe.to_string();
        let monitor = match monitor_maybe {
            site24x7_types::MonitorMaybe::URL(m)
            | site24x7_types::MonitorMaybe::HOMEPAGE(m)
            | site24x7_types::MonitorMaybe::REALBROWSER(m) => m,
            site24x7_types::MonitorMaybe::Unknown => continue,
        };
        for location in &monitor.locations {
            debug!(
                "Setting MONITOR_UP_GAUGE with {{monitor_type=\"{}\", \
                        monitor_name=\"{}\", monitor_group=\"{}\", location=\"{}\"}} \
                        to {}",
                &monitor_type,
                &monitor.name,
                &monitor_group,
                &location.location_name,
                location.clone().status as i64
            );
            MONITOR_UP_GAUGE
                .with_label_values(&[
                    &monitor_type,
                    &monitor.name,
                    &monitor_group,
                    &location.location_name,
                ])
                .set(location.clone().status as i64);

            // The original gauge is in milliseconds. Convert it to seconds first as prometheus wants
            // its time series data in seconds.
            MONITOR_LATENCY_SECONDS_GAUGE
                .with_label_values(&[
                    &monitor_type,
                    &monitor.name,
                    &monitor_group,
                    &location.location_name,
                ])
                // TODO Don't just set this to 0 but do it properly!
                .set(location.clone().attribute_value.unwrap_or(0) as f64 / 1000.0);
        }
    }
}

pub fn update_metrics_from_current_status(current_status_data: &CurrentStatusData) {
    // Update metrics based on the API data gathered above.

    // Reset these first so that we don't keep seeing any deleted monitors.
    MONITOR_UP_GAUGE.reset();
    MONITOR_LATENCY_SECONDS_GAUGE.reset();

    // Monitors can either be in a flat list of plain Monitors or they can be inside of a
    // MonitorGroup with is simply a list of monitors.
    set_metrics_for_monitors(&current_status_data.monitors, "");

    for monitor_group in &current_status_data.monitor_groups {
        set_metrics_for_monitors(&monitor_group.monitors, &monitor_group.group_name);
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use prometheus::{core::Collector, proto::MetricFamily};

    use crate::parsing::parse_current_status;

    use super::*;

    fn find_metric_location(
        metric_families: &Vec<MetricFamily>,
        metric_name: &str,
        location_name: &str,
    ) -> bool {
        if let Some(metric_families) = metric_families
            .iter()
            .find(|mf| mf.get_name() == metric_name)
        {
            metric_families
                .get_metric()
                .iter()
                .find(|m| {
                    m.get_label()
                        .iter()
                        .any(|l| l.get_name() == "location" && l.get_value() == location_name)
                })
                .is_some()
        } else {
            false
        }
    }

    #[test]
    /// If we get an entirely empty body, we don't want to see any metrics getting created.
    fn no_metrics_are_created_if_empty_body() -> Result<()> {
        let data = parse_current_status(include_str!("../tests/data/empty_response.json"))?;
        update_metrics_from_current_status(&data);
        assert!(prometheus::gather().is_empty());
        Ok(())
    }

    #[test]
    /// A simple case where we expect to find two locations in the output.
    fn simple_two_locations() -> Result<()> {
        let data = parse_current_status(include_str!("../tests/data/simple_two_locations.json"))?;
        update_metrics_from_current_status(&data);
        let metrics = prometheus::gather();
        assert!(find_metric_location(
            &metrics,
            "site24x7_monitor_latency_seconds",
            "Bucharest - RO"
        ));
        // TODO Important the tests run in threads and access the global registry which
        // means we'll get unsafe side effects from other test runs!
        assert!(find_metric_location(
            &metrics,
            "site24x7_monitor_latency_seconds",
            "London - UK"
        ));
        Ok(())
    }

    #[test]
    /// A removed location should disappear.
    fn removed_location_should_disappear() -> Result<()> {
        let data_before =
            parse_current_status(include_str!("../tests/data/simple_two_locations.json"))?;
        let data_after =
            parse_current_status(include_str!("../tests/data/simple_one_location.json"))?;

        // We'll update metrics twice here. `data_before` has two locations while
        // `data_after` only has one location. We therefore expect the output to only contain a
        // single location.
        update_metrics_from_current_status(&data_before);
        update_metrics_from_current_status(&data_after);
        let metrics = prometheus::gather();

        assert!(find_metric_location(
            &metrics,
            "site24x7_monitor_latency_seconds",
            "Bucharest - RO"
        ));
        // Expect the London locatio to be gone.
        assert!(!find_metric_location(
            &metrics,
            "site24x7_monitor_latency_seconds",
            "London - UK"
        ));
        Ok(())
    }
}
