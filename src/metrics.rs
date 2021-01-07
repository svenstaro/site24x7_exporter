//! Module containing functions related to handling metrics.
use std::collections::HashMap;

use log::{debug, info};
use prometheus::proto::MetricFamily;

use crate::{
    site24x7_types::{self, CurrentStatusData},
    MONITOR_LATENCY_SECONDS_GAUGE, MONITOR_UP_GAUGE,
};

/// Set the Prometheus metrics for `monitors`.
///
/// Set `monitor_group` to `""` in case the monitor doesn't belong to a monitor group on Site24x7.
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
                "Setting site24x7_monitor_up{{monitor_type=\"{}\",monitor_name=\"{}\",monitor_group=\"{}\",location=\"{}\"}} {}",
                &monitor_type,
                &monitor.name,
                &monitor_group,
                &location.location_name,
                location.clone().status as i64
            );
            let up_gauge = MONITOR_UP_GAUGE.with_label_values(&[
                &monitor_type,
                &monitor.name,
                &monitor_group,
                &location.location_name,
            ]);
            up_gauge.set(location.clone().status as i64);

            // There is a special case where sometimes locations don't report an
            // `attribute_value` even though they are up. This appears to happen
            // in case monitor hasn't managed to poll new data for some time.
            // Frankly it's not great that Site24x7 does this but they do and so we've got to
            // deal with it somehow.
            // It doesn't really make sense to integrate an non-value as the monitor would
            // receive a value of 0 in that case so we'll just skip it.
            // Ideally, this results in us reporting the last value in case there already was
            // one from before which is good enough.
            if location.attribute_value.is_none() && location.status == site24x7_types::Status::Up {
                continue;
            }

            // The original gauge is in milliseconds. Convert it to seconds first as prometheus wants
            // its time series data in seconds.
            let attribute_value = if let Some(attribute_value) = location.attribute_value {
                attribute_value as f64 / 1000.0
            } else if location.status != site24x7_types::Status::Up {
                // We'll report NaN instead of 0 if the monitor is down as a latency of 0 might
                // be misleading.
                // See https://prometheus.io/docs/practices/instrumentation/#avoid-missing-metrics
                f64::NAN
            } else {
                0.0
            };
            debug!(
                "Setting site24x7_monitor_latency_seconds{{monitor_type=\"{}\",monitor_name=\"{}\",monitor_group=\"{}\",location=\"{}\"}} {}",
                &monitor_type,
                &monitor.name,
                &monitor_group,
                &location.location_name,
                attribute_value,
            );
            let latency_gauge = MONITOR_LATENCY_SECONDS_GAUGE.with_label_values(&[
                &monitor_type,
                &monitor.name,
                &monitor_group,
                &location.location_name,
            ]);
            latency_gauge.set(attribute_value);
        }
    }
}

/// Return whether `monitors` contains a monitor having given attributes.
fn has_monitor_with_label_values(
    monitors: &[site24x7_types::MonitorMaybe],
    monitor_type: &str,
    monitor_name: &str,
    location_name: &str,
) -> bool {
    for monitor_maybe in monitors {
        let monitor = match monitor_maybe {
            site24x7_types::MonitorMaybe::URL(m)
            | site24x7_types::MonitorMaybe::HOMEPAGE(m)
            | site24x7_types::MonitorMaybe::REALBROWSER(m) => m,
            site24x7_types::MonitorMaybe::Unknown => continue,
        };
        for location in &monitor.locations {
            if monitor_type == monitor_maybe.to_string()
                && monitor_name == monitor.name
                && location_name == location.location_name
            {
                return true;
            }
        }
    }
    false
}

/// Clean up metrics that were deleted or somehow became invalid.
fn cleanup_metrics_for_monitors(
    metric_families: &[MetricFamily],
    monitors: &[site24x7_types::MonitorMaybe],
    monitor_group: &str,
) {
    for metric_family in metric_families {
        for metric in metric_family.get_metric() {
            // Skip any metrics that are not in the given `monitor_group`.
            let current_monitor_group = metric
                .get_label()
                .iter()
                .find(|l| l.get_name() == "monitor_group")
                .unwrap()
                .get_value();
            if current_monitor_group != monitor_group {
                continue;
            }
            let monitor_type = metric
                .get_label()
                .iter()
                .find(|l| l.get_name() == "monitor_type")
                .unwrap()
                .get_value();
            let monitor_name = metric
                .get_label()
                .iter()
                .find(|l| l.get_name() == "monitor_name")
                .unwrap()
                .get_value();
            let location_name = metric
                .get_label()
                .iter()
                .find(|l| l.get_name() == "location")
                .unwrap()
                .get_value();
            if !has_monitor_with_label_values(monitors, monitor_type, monitor_name, location_name) {
                let mut labels = HashMap::new();
                labels.insert("monitor_type", monitor_type);
                labels.insert("monitor_name", monitor_name);
                labels.insert("monitor_group", monitor_group);
                labels.insert("location", location_name);
                if metric_family.get_name() == "site24x7_monitor_up" {
                    info!("Cleaning up now-missing metric site24x7_monitor_up{{monitor_type=\"{}\",monitor_name=\"{}\",monitor_group=\"{}\",location=\"{}\"}}",
                        monitor_type,
                        monitor_name,
                        monitor_group,
                        location_name,
                    );
                    MONITOR_UP_GAUGE.remove(&labels).unwrap();
                } else if metric_family.get_name() == "site24x7_monitor_latency_seconds" {
                    info!("Cleaning up now-missing metric site24x7_monitor_latency_seconds{{monitor_type=\"{}\",monitor_name=\"{}\",monitor_group=\"{}\",location=\"{}\"}}",
                        monitor_type,
                        monitor_name,
                        monitor_group,
                        location_name,
                    );
                    MONITOR_LATENCY_SECONDS_GAUGE.remove(&labels).unwrap();
                }
            }
        }
    }
}

/// Update metrics based on previously gathered data from /current_status API.
pub fn update_metrics_from_current_status(current_status_data: &CurrentStatusData) {
    // Clean up monitors that were removed.
    let metric_families = prometheus::gather();

    cleanup_metrics_for_monitors(&metric_families, &current_status_data.monitors, "");
    for monitor_group in &current_status_data.monitor_groups {
        cleanup_metrics_for_monitors(
            &metric_families,
            &monitor_group.monitors,
            &monitor_group.group_name,
        );
    }

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
    use prometheus::{Encoder, TextEncoder};

    use crate::parsing::parse_current_status;

    use super::*;

    /// Since we're using the global exporter, tests can influence and will influence eachother via
    /// global state. We'll therefore have to call this function before every test to make sure we
    /// start with a clean slate.
    fn clear_state() {
        MONITOR_UP_GAUGE.reset();
        MONITOR_LATENCY_SECONDS_GAUGE.reset();
    }

    /// Return whether `metric_name` has a label `label_name` having `label_value` in a list `metric_families`.
    fn has_label_with_value(
        metric_families: &Vec<MetricFamily>,
        metric_name: &str,
        label_name: &str,
        label_value: &str,
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
                        .any(|l| l.get_name() == label_name && l.get_value() == label_value)
                })
                .is_some()
        } else {
            false
        }
    }

    #[test]
    /// If we get an entirely empty body, we don't want to see any metrics getting created.
    fn no_metrics_are_created_if_empty_body() -> Result<()> {
        clear_state();
        let data = parse_current_status(include_str!("../tests/data/empty_response.json"))?;
        update_metrics_from_current_status(&data);
        assert!(prometheus::gather().is_empty());
        Ok(())
    }

    #[test]
    /// A simple case where we expect to find two locations in the output.
    fn simple_two_locations() -> Result<()> {
        clear_state();
        let data = parse_current_status(include_str!("../tests/data/simple_two_locations.json"))?;
        update_metrics_from_current_status(&data);
        assert_eq!(
            MONITOR_UP_GAUGE
                .with_label_values(&["URL", "test", "", "London - UK"])
                .get(),
            1
        );
        assert_eq!(
            MONITOR_UP_GAUGE
                .with_label_values(&["URL", "test", "", "Bucharest - RO"])
                .get(),
            1
        );
        assert_eq!(
            MONITOR_LATENCY_SECONDS_GAUGE
                .with_label_values(&["URL", "test", "", "London - UK"])
                .get(),
            0.421
        );
        assert_eq!(
            MONITOR_LATENCY_SECONDS_GAUGE
                .with_label_values(&["URL", "test", "", "Bucharest - RO"])
                .get(),
            0.757
        );
        Ok(())
    }

    #[test]
    /// A removed location should disappear.
    fn removed_location_should_disappear() -> Result<()> {
        clear_state();
        let data_before =
            parse_current_status(include_str!("../tests/data/simple_two_locations.json"))?;
        let data_after =
            parse_current_status(include_str!("../tests/data/simple_one_location.json"))?;

        // We'll update metrics twice here. `data_before` has two locations while
        // `data_after` only has one location. We therefore expect the output to only contain a
        // single location.
        update_metrics_from_current_status(&data_before);
        update_metrics_from_current_status(&data_after);
        let metric_families = prometheus::gather();

        assert!(has_label_with_value(
            &metric_families,
            "site24x7_monitor_latency_seconds",
            "location",
            "Bucharest - RO"
        ));
        // Expect the London locatio to be gone.
        assert!(!has_label_with_value(
            &metric_families,
            "site24x7_monitor_latency_seconds",
            "location",
            "London - UK"
        ));
        Ok(())
    }

    #[test]
    /// A removed monitor should disappear.
    fn removed_monitors_should_disappear() -> Result<()> {
        clear_state();
        let data_before =
            parse_current_status(include_str!("../tests/data/simple_two_monitors.json"))?;
        let data_after =
            parse_current_status(include_str!("../tests/data/simple_one_monitor.json"))?;

        // We'll update metrics twice here. `data_before` has two monitors while
        // `data_after` only has one monitor. We therefore expect the output to only contain a
        // single monitor.
        update_metrics_from_current_status(&data_before);
        update_metrics_from_current_status(&data_after);
        let metric_families = prometheus::gather();

        assert!(has_label_with_value(
            &metric_families,
            "site24x7_monitor_latency_seconds",
            "monitor_name",
            "test1"
        ));
        // Expect the test2 monitor to be gone.
        assert!(!has_label_with_value(
            &metric_families,
            "site24x7_monitor_latency_seconds",
            "monitor_name",
            "test2"
        ));
        Ok(())
    }

    #[test]
    /// An update that contains a monitor with a location that doesn't have `attribute_value`
    /// set should not overwrite an existing metric with the same labels.
    ///
    /// This case sometimes appears to happen when a location hasn't been fetched in a while
    /// which will cause it to not report an `attribute_value`.
    /// It's better to keep the old value in that case.
    fn keep_old_value_if_update_is_invalid() -> Result<()> {
        clear_state();
        let data_before =
            parse_current_status(include_str!("../tests/data/simple_two_locations.json"))?;
        let data_after =
            parse_current_status(include_str!("../tests/data/no_attribute_value.json"))?;

        // We'll update metrics twice here. `data_before` has two valid locations that both
        // report their data properly while
        // `data_after` has one location that stops reporting its `attribute_value`.
        // We therefore expect the output after the second update to not be changed.
        update_metrics_from_current_status(&data_before);
        assert_eq!(
            MONITOR_LATENCY_SECONDS_GAUGE
                .with_label_values(&["URL", "test", "", "London - UK"])
                .get(),
            0.421
        );

        update_metrics_from_current_status(&data_after);
        assert_eq!(
            MONITOR_LATENCY_SECONDS_GAUGE
                .with_label_values(&["URL", "test", "", "London - UK"])
                .get(),
            0.421
        );

        Ok(())
    }

    #[test]
    /// Monitors that are down should report NaN as their latency value.
    ///
    /// See https://prometheus.io/docs/practices/instrumentation/#avoid-missing-metrics
    fn report_nan_for_down_monitor() -> Result<()> {
        clear_state();
        let data = parse_current_status(include_str!("../tests/data/down_monitor.json"))?;
        update_metrics_from_current_status(&data);
        assert_eq!(
            MONITOR_LATENCY_SECONDS_GAUGE
                .with_label_values(&["URL", "test", "", "London - UK"])
                .get(),
            27.458
        );
        assert!(MONITOR_LATENCY_SECONDS_GAUGE
            .with_label_values(&["URL", "test", "", "Bucharest - RO"])
            .get()
            .is_nan());

        Ok(())
    }

    #[test]
    /// Check that there are no changes between two identical status updates.
    fn identical_update_no_changes() -> Result<()> {
        clear_state();
        let s = include_str!("../tests/data/full.json");
        let data = parse_current_status(s)?;
        update_metrics_from_current_status(&data);
        let mut before = vec![];
        let encoder = TextEncoder::new();
        encoder.encode(&prometheus::gather(), &mut before).unwrap();
        update_metrics_from_current_status(&data);
        let mut after = vec![];
        let encoder = TextEncoder::new();
        encoder.encode(&prometheus::gather(), &mut after).unwrap();
        assert_eq!(before, after);
        Ok(())
    }
}
