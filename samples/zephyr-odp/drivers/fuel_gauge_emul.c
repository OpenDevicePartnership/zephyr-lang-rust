/*
 * Copyright (c) 2026 John Computer
 * SPDX-License-Identifier: Apache-2.0
 *
 * Emulated fuel gauge driver for testing without hardware.
 */

#define DT_DRV_COMPAT zephyr_fuel_gauge_emul

#include <zephyr/device.h>
#include <zephyr/drivers/fuel_gauge.h>
#include <zephyr/kernel.h>
#include <zephyr/logging/log.h>
#include <string.h>

LOG_MODULE_REGISTER(fuel_gauge_emul, CONFIG_FUEL_GAUGE_LOG_LEVEL);

struct fuel_gauge_emul_data {
	/* Mutable state - can be modified via set_property */
	uint16_t sbs_mode;
};

static int fuel_gauge_emul_get_property(const struct device *dev,
					fuel_gauge_prop_t prop,
					union fuel_gauge_prop_val *val)
{
	struct fuel_gauge_emul_data *data = dev->data;

	switch (prop) {
	case FUEL_GAUGE_VOLTAGE:
		val->voltage = 3700000; /* 3.7V in µV */
		break;
	case FUEL_GAUGE_CURRENT:
	case FUEL_GAUGE_AVG_CURRENT:
		val->current = -100000; /* -100mA in µA (discharging) */
		break;
	case FUEL_GAUGE_TEMPERATURE:
		val->temperature = 2981; /* 25°C in dK (298.1K) */
		break;
	case FUEL_GAUGE_ABSOLUTE_STATE_OF_CHARGE:
	case FUEL_GAUGE_RELATIVE_STATE_OF_CHARGE:
		val->relative_state_of_charge = 75; /* 75% */
		break;
	case FUEL_GAUGE_REMAINING_CAPACITY:
		val->remaining_capacity = 2250000; /* 2250mAh in µAh (75% of 3000mAh) */
		break;
	case FUEL_GAUGE_FULL_CHARGE_CAPACITY:
		val->full_charge_capacity = 3000000; /* 3000mAh in µAh */
		break;
	case FUEL_GAUGE_RUNTIME_TO_EMPTY:
		val->runtime_to_empty = 1350; /* ~22.5 hours in minutes */
		break;
	case FUEL_GAUGE_RUNTIME_TO_FULL:
		val->runtime_to_full = 0xFFFF; /* Not charging */
		break;
	case FUEL_GAUGE_CHARGE_VOLTAGE:
		val->chg_voltage = 4200000; /* 4.2V in µV */
		break;
	case FUEL_GAUGE_CHARGE_CURRENT:
		val->chg_current = 1000000; /* 1A in µA */
		break;
	case FUEL_GAUGE_DESIGN_CAPACITY:
		val->design_cap = 3000; /* 3000mAh */
		break;
	case FUEL_GAUGE_DESIGN_VOLTAGE:
		val->design_volt = 3700; /* 3.7V in mV */
		break;
	case FUEL_GAUGE_CYCLE_COUNT:
		val->cycle_count = 42;
		break;
	case FUEL_GAUGE_SBS_MODE:
		val->sbs_mode = data->sbs_mode;
		break;
	case FUEL_GAUGE_STATUS:
		val->fg_status = 0;
		break;
	case FUEL_GAUGE_PRESENT_STATE:
		val->flags = 1; /* Battery present */
		break;
	case FUEL_GAUGE_CONNECT_STATE:
		val->flags = 1; /* Battery connected */
		break;
	case FUEL_GAUGE_FLAGS:
		val->flags = 0; /* No error flags */
		break;
	case FUEL_GAUGE_STATE_OF_HEALTH:
		val->state_of_health = 100; /* Perfect health */
		break;
	default:
		LOG_DBG("Unsupported property %d", prop);
		return -ENOTSUP;
	}

	return 0;
}

static int fuel_gauge_emul_set_property(const struct device *dev,
					fuel_gauge_prop_t prop,
					union fuel_gauge_prop_val val)
{
	struct fuel_gauge_emul_data *data = dev->data;

	switch (prop) {
	case FUEL_GAUGE_SBS_MODE:
		data->sbs_mode = val.sbs_mode;
		break;
	case FUEL_GAUGE_SBS_REMAINING_CAPACITY_ALARM:
	case FUEL_GAUGE_SBS_REMAINING_TIME_ALARM:
	case FUEL_GAUGE_SBS_ATRATE:
		/* Accept but ignore */
		break;
	default:
		LOG_DBG("Unsupported set property %d", prop);
		return -ENOTSUP;
	}

	return 0;
}

static int fuel_gauge_emul_get_buffer_property(const struct device *dev,
					       fuel_gauge_prop_t prop,
					       void *dst, size_t dst_len)
{
	switch (prop) {
	case FUEL_GAUGE_MANUFACTURER_NAME:
		if (dst_len < 8) {
			return -ENOBUFS;
		}
		memcpy(dst, "Emulated", 8);
		return 8;
	case FUEL_GAUGE_DEVICE_NAME:
		if (dst_len < 12) {
			return -ENOBUFS;
		}
		memcpy(dst, "FG-Emulator", 11);
		return 11;
	case FUEL_GAUGE_DEVICE_CHEMISTRY:
		if (dst_len < 5) {
			return -ENOBUFS;
		}
		memcpy(dst, "LION", 4);
		return 4;
	default:
		return -ENOTSUP;
	}
}

static int fuel_gauge_emul_battery_cutoff(const struct device *dev)
{
	LOG_INF("Battery cutoff requested (emulated - no action)");
	return 0;
}

static int fuel_gauge_emul_init(const struct device *dev)
{
	struct fuel_gauge_emul_data *data = dev->data;

	data->sbs_mode = 0;

	LOG_INF("Fuel gauge emulator initialized");
	return 0;
}

static DEVICE_API(fuel_gauge, fuel_gauge_emul_api) = {
	.get_property = fuel_gauge_emul_get_property,
	.set_property = fuel_gauge_emul_set_property,
	.get_buffer_property = fuel_gauge_emul_get_buffer_property,
	.battery_cutoff = fuel_gauge_emul_battery_cutoff,
};

#define FUEL_GAUGE_EMUL_INIT(inst)                                              \
	static struct fuel_gauge_emul_data fuel_gauge_emul_data_##inst;             \
	DEVICE_DT_INST_DEFINE(inst, fuel_gauge_emul_init, NULL,                     \
			      &fuel_gauge_emul_data_##inst,                         \
			      NULL,                                                 \
			      POST_KERNEL, CONFIG_FUEL_GAUGE_INIT_PRIORITY,         \
			      &fuel_gauge_emul_api);

DT_INST_FOREACH_STATUS_OKAY(FUEL_GAUGE_EMUL_INIT)
