# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

set(DTS_ROOT ${CMAKE_CURRENT_LIST_DIR})

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})

project(odp_thermal_service)
rust_cargo_application()

target_sources(app PRIVATE drivers/mock_temp_sensor/mock_temp_sensor.c)
