/*
 * Copyright (c) 2019 Centaur Analytics, Inc
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>
#include <zephyr/device.h>
#include <zephyr/drivers/sensor.h>
#include <zephyr/drivers/eeprom.h>
#include <zephyr/drivers/sensor/tmp11x.h>
#include <zephyr/sys/printk.h>
#include <zephyr/sys/__assert.h>

#define TMP11X_NODE DT_COMPAT_GET_ANY_STATUS_OKAY(ti_tmp11x)

void tmp11x_read(void *dummy1, void *dummy2, void *dummy3)
{
    const struct device *const dev = DEVICE_DT_GET(TMP11X_NODE);
    struct sensor_value temp_value;

    /* offset to be added to the temperature
     * only supported by TMP117 and TMP119
     */
    struct sensor_value offset_value;
    int ret;

    __ASSERT(device_is_ready(dev), "tmp11x device not ready");
    __ASSERT(device_is_ready(eeprom), "tmp11x eeprom device not ready");

    printk("Device %s - %p is ready\n", dev->name, dev);

    /*
     * if an offset of 2.5 oC is to be added,
     * set val1 = 2 and val2 = 500000.
     * See struct sensor_value documentation for more details.
     */
    offset_value.val1 = 0;
    offset_value.val2 = 0;
    ret = sensor_attr_set(dev, SENSOR_CHAN_AMBIENT_TEMP,
                          SENSOR_ATTR_OFFSET, &offset_value);
    if (ret)
    {
        printk("sensor_attr_set failed ret = %d\n", ret);
        printk("SENSOR_ATTR_OFFSET is only supported by TMP117 and TMP119\n");
    }
    while (1)
    {
        ret = sensor_sample_fetch(dev);
        if (ret)
        {
            printk("Failed to fetch measurements (%d)\n", ret);
            k_sleep(K_MSEC(1000));
            continue;
        }

        ret = sensor_channel_get(dev, SENSOR_CHAN_AMBIENT_TEMP,
                                 &temp_value);
        if (ret)
        {
            printk("Failed to get measurements (%d)\n", ret);
            k_sleep(K_MSEC(1000));
            continue;
        }

        printk("Temperature read in C\n");
        printk("   Temp is %d.%d Celcius\n", temp_value.val1, temp_value.val2);

        k_sleep(K_MSEC(1000));
    }

    return;
}
