# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

# Add custom DTS bindings directory
list(APPEND DTS_ROOT ${CMAKE_CURRENT_SOURCE_DIR})

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})

project(odp_thermal_service)
rust_cargo_application()

# Compile the C mock drivers
target_sources(app PRIVATE drivers/fuel_gauge_emul.c)
target_sources(app PRIVATE drivers/pwm_emul.c)