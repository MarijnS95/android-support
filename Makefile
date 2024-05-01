android_jar = ~/Android/Sdk/platforms/android-34/android.jar
java_target = 17 # Matches release 34

classes.dex: java/*.class
	d8 --min-api 21 $^

java/*.class: java/*.java | $(android_jar)
	javac --release $(java_target) $^ -cp $|
