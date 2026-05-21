/*
 * Copyright (c) 2025
 * SPDX-License-Identifier: Apache-2.0
 *
 * Mock temperature sensor driver for testing without hardware.
 * Produces a simple temperature ramp from 25.0°C to 45.0°C.
 */

#define DT_DRV_COMPAT zephyr_mock_temp_sensor

#include <zephyr/device.h>
#include <zephyr/drivers/sensor.h>
#include <zephyr/logging/log.h>

LOG_MODULE_REGISTER(mock_temp_sensor, CONFIG_SENSOR_LOG_LEVEL);

struct mock_temp_data {
	struct sensor_value temperature;
	int32_t step;
};

static int mock_temp_sample_fetch(const struct device *dev, enum sensor_channel chan)
{
	struct mock_temp_data *data = dev->data;

	if (chan != SENSOR_CHAN_ALL && chan != SENSOR_CHAN_AMBIENT_TEMP) {
		return -ENOTSUP;
	}

	/* Ramp from 25.0 to 45.0 in 0.5 degree steps, then wrap */
	int32_t val1 = 25 + (data->step / 2);
	int32_t val2 = (data->step % 2) * 500000;

	data->temperature.val1 = val1;
	data->temperature.val2 = val2;

	data->step++;
	if (data->step >= 40) {
		data->step = 0;
	}

	LOG_DBG("Mock temp: %d.%06d C", val1, val2);

	return 0;
}

static int mock_temp_channel_get(const struct device *dev, enum sensor_channel chan,
				 struct sensor_value *val)
{
	struct mock_temp_data *data = dev->data;

	if (chan != SENSOR_CHAN_AMBIENT_TEMP) {
		return -ENOTSUP;
	}

	*val = data->temperature;
	return 0;
}

static int mock_temp_attr_set(const struct device *dev, enum sensor_channel chan,
			      enum sensor_attribute attr, const struct sensor_value *val)
{
	/* Accept any attribute set as a no-op */
	return 0;
}

static DEVICE_API(sensor, mock_temp_api) = {
	.sample_fetch = mock_temp_sample_fetch,
	.channel_get = mock_temp_channel_get,
	.attr_set = mock_temp_attr_set,
};

static int mock_temp_init(const struct device *dev)
{
	struct mock_temp_data *data = dev->data;

	data->temperature.val1 = 25;
	data->temperature.val2 = 0;
	data->step = 0;

	LOG_INF("Mock temperature sensor initialized");
	return 0;
}

#define MOCK_TEMP_DEFINE(inst)                                              \
	static struct mock_temp_data mock_temp_data_##inst;                 \
	SENSOR_DEVICE_DT_INST_DEFINE(inst, mock_temp_init, NULL,           \
				     &mock_temp_data_##inst, NULL,          \
				     POST_KERNEL, CONFIG_SENSOR_INIT_PRIORITY, \
				     &mock_temp_api);

DT_INST_FOREACH_STATUS_OKAY(MOCK_TEMP_DEFINE)
