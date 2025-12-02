/*
 * Copyright (c) 2012-2014 Wind River Systems, Inc.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>

extern void rust_main(void);

#if defined(CONFIG_LOG) && !defined(CONFIG_LOG_MINIMAL)
// Logging, to see how things things are expanded.
#include <zephyr/logging/log.h>
LOG_MODULE_REGISTER(rust, 3);

void rust_log_message(uint32_t level, char *msg)
{
    // Ok.  The log macros in Zephyr perform all kinds of macro stitching, etc, on the
    // arguments.  As such, we can't just pass the level to something, but actually need to
    // expand things here.  This puts the file and line information of the log in this file,
    // rather than where we came from.
    switch (level)
    {
    case LOG_LEVEL_ERR:
        LOG_ERR("%s", msg);
        break;
    case LOG_LEVEL_WRN:
        LOG_WRN("%s", msg);
        break;
    case LOG_LEVEL_INF:
        LOG_INF("%s", msg);
        break;
    case LOG_LEVEL_DBG:
    default:
        LOG_DBG("%s", msg);
        break;
    }
}
#endif /* defined(CONFIG_LOG) && !defined(CONFIG_LOG_MINIMAL) */

struct k_thread c_app_thread;
K_THREAD_STACK_DEFINE(c_app_stack, CONFIG_MAIN_STACK_SIZE);
extern void tmp11x_read(void *dummy1, void *dummy2, void *dummy3);

int main(void)
{
    LOG_WRN("W: Custom C entry point! %s\n", CONFIG_BOARD_TARGET);

#ifdef CONFIG_RUST
    // Rust implementation of temperature read
    rust_main();
#else
    // C implementation of temperature read
    k_thread_create(&c_app_thread, c_app_stack, CONFIG_MAIN_STACK_SIZE,
                    tmp11x_read, NULL, NULL, NULL,
                    -1, K_USER, K_MSEC(0));
#endif

    return 0;
}

/* On most arches, panic is entirely macros resulting in some kind of inline assembly.  Create this
 * wrapper so the Rust panic handler can call the same kind of panic.
 */
void rust_panic_wrap(void)
{
    k_panic();
}
