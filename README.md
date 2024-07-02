# Conversion

//1.生成文件

android Conversion -t ./android/module1/values/strings.xml -t ./android/module2/values/strings.xml
ios Conversion -t ./ios/module1/Localizable.strings -t ./ios/module2/Localizable.strings
java Conversion -m ./to_android.xlsx -m ./to_ios.xlsx -m ./to_java.xlsx
//2.合并文件 Conversion.exe --mode merge
//3.解析文件 Conversion.exe --mode out --platform android
//3.解析文件 Conversion.exe --mode out --platform android

