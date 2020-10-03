### display test on PyBadge

A short delay added before the display initialization seems to help with the following problem: 
after power up the display is white (just the backlight), it requires one or more resets to show the desired content.
This seems to fix it, just like a similar issue on the SSD1306 I2C display.
