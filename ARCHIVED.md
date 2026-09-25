# Archived — not the shipping firmware

This ESP32-C3 build is **not maintained** and is **not a shipping target**.

The product ships on the **STM32F103C8T6**. Its firmware is the sibling export
`STM32F103_mw_radar_led_fade_encoder_display`. Fixes and features go there, not
here.

## Why this folder still exists

It is the same device implemented on RISC-V — mmWave radar over UART, SSD1306
128×32 over I²C, PWM-faded LED, encoder-driven menu — using `esp-hal` and the
async HAL. That is the only place that implementation exists, and throwing it
away would lose the async version of the driver work for nothing. It is kept to
read, not to build.

If you are here because you found two folders that looked equally current: that
is exactly what this file is for.

## Where the decision lives

Recorded on the product board as `software#16`, answering `software#12`, which
in turn followed `hardware#8` — the decision to ship on STM32F103.

```bash
orch show software        # the decision and its links
orch brief software       # the whole domain, cold
```

The board is at `F:\embedded_product_to_business_platform`.

Archived 2026-09-09.
