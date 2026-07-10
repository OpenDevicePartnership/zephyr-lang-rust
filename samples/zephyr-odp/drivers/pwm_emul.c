/*
 * Copyright (c) 2026
 * SPDX-License-Identifier: Apache-2.0
 *
 * Emulated PWM controller for testing without hardware.
 *
 * This is a small functional stub (not an FFF mock like zephyr,fake-pwm). Also unlike zephyr,fake-pwm, it
 * uses the standard 3-cell PWM specifier (channel, period, flags) instead of 2.
 */

#define DT_DRV_COMPAT zephyr_pwm_emul

#include <zephyr/device.h>
#include <zephyr/drivers/pwm.h>
#include <zephyr/kernel.h>
#include <zephyr/logging/log.h>

LOG_MODULE_REGISTER(pwm_emul, CONFIG_PWM_LOG_LEVEL);

#define PWM_EMUL_NUM_CHANNELS 8

struct pwm_emul_config {
	uint64_t frequency_hz;
};

struct pwm_emul_channel {
	uint32_t period_cycles;
	uint32_t pulse_cycles;
	pwm_flags_t flags;
};

struct pwm_emul_data {
	struct pwm_emul_channel channels[PWM_EMUL_NUM_CHANNELS];
};

static int pwm_emul_set_cycles(const struct device *dev, uint32_t channel,
			       uint32_t period_cycles, uint32_t pulse_cycles,
			       pwm_flags_t flags)
{
	struct pwm_emul_data *data = dev->data;

	if (channel >= PWM_EMUL_NUM_CHANNELS) {
		return -EINVAL;
	}

	data->channels[channel].period_cycles = period_cycles;
	data->channels[channel].pulse_cycles = pulse_cycles;
	data->channels[channel].flags = flags;

	LOG_DBG("channel %u: period=%u pulse=%u flags=0x%x", channel,
		period_cycles, pulse_cycles, (unsigned int)flags);

	return 0;
}

static int pwm_emul_get_cycles_per_sec(const struct device *dev,
				       uint32_t channel, uint64_t *cycles)
{
	const struct pwm_emul_config *config = dev->config;

	ARG_UNUSED(channel);

	*cycles = config->frequency_hz;

	return 0;
}

static DEVICE_API(pwm, pwm_emul_api) = {
	.set_cycles = pwm_emul_set_cycles,
	.get_cycles_per_sec = pwm_emul_get_cycles_per_sec,
};

static int pwm_emul_init(const struct device *dev)
{
	ARG_UNUSED(dev);
	return 0;
}

#define PWM_EMUL_INIT(inst)                                                    \
	static struct pwm_emul_data pwm_emul_data_##inst;                       \
	static const struct pwm_emul_config pwm_emul_config_##inst = {          \
		.frequency_hz = DT_INST_PROP(inst, frequency),                 \
	};                                                                     \
	DEVICE_DT_INST_DEFINE(inst, pwm_emul_init, NULL,                       \
			      &pwm_emul_data_##inst, &pwm_emul_config_##inst,  \
			      POST_KERNEL, CONFIG_PWM_INIT_PRIORITY,           \
			      &pwm_emul_api);

DT_INST_FOREACH_STATUS_OKAY(PWM_EMUL_INIT)
