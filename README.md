# STM32 を使った SSD1306 接続例

STM32C092KCT6 から OLED ディスプレイ SSD1306 に接続する例です。STM32CubeIDE プロジェクトです。

以下の配線で I2C2 から制御します。

![配線図](doc/wiring.png)

プロジェクトをビルドするには、ライブラリ stm32-ssd1306 が必要です。

https://github.com/afiskon/stm32-ssd1306

このプロジェクトをチェックアウトした後、以下のコマンドを実行するとライブラリがチェックアウトされます。

```
git update --init --recursive
```

馬のアニメーションは ControllersTech のチュートリアルで配布されているものを使いました。
https://controllerstech.com/oled-display-using-i2c-stm32/
