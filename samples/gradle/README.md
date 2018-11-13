## Sample Gradle project

Most interesting for you here is not the source code, but configs in [`.mainframer`](.mainframer) folder.

This is a very common setup for basically any Gradle project (including multimodule), but of course you might need to tune some ignore configs for your project.

### How to build

```bash
$ mainframer ./gradlew build
```

Or any other Gradle task you want, btw it integrates with IntelliJ pretty easily! TODO add link to docs.

### Requirements

JDK 6+ installed on remote machine.

### Recipes

* [IntelliJ Run Config Recipe](../../recipes/INTELLIJ_RUN_CONFIG.md)
