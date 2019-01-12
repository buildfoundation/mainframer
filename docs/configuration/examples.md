# Configuration Examples

## `ignore.yml`

### General

```yaml
push:
  # No need to sync build results to the remote machine.
  # Replace "out" with the directory where build results are placed.
  - "out"
pull:
  # No need to sync sources to the local machine.
  # Replace "src" with the directory where the source code is placed.
  - "src"
```

### Tools

#### Git

```yaml
push:
  # Synching ".git" or other VCS directories is a bad idea.
  # They are usually heavy and are not required for the build.
  - "/.git"
```

#### IntelliJ IDEA

Includes Android Studio, PyCharm, WebStorm, GoLand and other tools built
on the IntelliJ Platform.

```yaml
push:
  # Synching IDE-specific directories is not required for the build.
  - "/.idea"
```

### Build Systems

#### Buck

```yaml
push:
  # No need to sync build results to the remote machine.
  - "buck-out"
both:
  # ".buckd" contains machine-specific files, no need to sync it between machines.
  - "/.buckd"
```

#### Cargo

```yaml
push:
  # No need to sync build resuls to the remote machine.
  - "target"
both:
  # Remove Cargo.lock if creating an executable, leave it for libraries.
  # More information: http://doc.crates.io/guide.html#cargotoml-vs-cargolock
  - "/Cargo.lock"
```

#### Go

```yaml
push:
  # No need to sync build results to the remote machine.
  - "bin"
```

#### Gradle

```yaml
push:
  # No need to sync build results to the remote machine.
  - "build"
both:
  # ".gradle" contains machine-specific files, no need to sync it between machines.
  - ".gradle"
```

#### Gradle: Android

Gradle rules apply as well.

```yaml
push:
  # Syncing captures from Android Studio is usually not required.
  - "/captures"
both:
  # Synching "local.properties" is a bad idea since it contains
  # machine-specific values like paths to SDKs.
  - "/local.properties"
```

#### Maven

```yaml
push:
  # No need to sync build results to the remote machine.
  - "target"
```
