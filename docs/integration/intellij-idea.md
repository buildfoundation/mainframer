# Integration with IntelliJ IDEA

Both plugin and plugin-less integrations use Run Configurations
to apply Mainframer as a Make step. This is not a hack,
aparently IDE supports this for years. Since Run Configurations
are flexible it is possible to do different things.

* Create a new Configuration for Mainframer-specific purposes.
    * Example: creating a Configuration A for building on a local machine
      and a Configuration B for building on a remote machine.
* Change a Configuration Template to apply Mainframer
  to all derivative configurations.
    * Example: changing a JUnit template to apply Mainframer
      to all JUnit runs.

## :warning: Android Studio

Android Studio tends to break the IDE contract and injects
Gradle-aware Make step even if Mainframer step was explicitly declared.
This happens on every Gradle Sync operation.
Please refer to
[the Android Studio issue](https://issuetracker.google.com/issues/77840239#comment3)
for details.

## Plugin

There is [a third-party plugin](https://github.com/elpassion/mainframer-intellij-plugin),
please refer to the project documentation for installation and advanced instructions.

The plugin is not required though, it is completely possible to use Mainframer without it.

## Instructions

It is important to determine beforehand which console command should be invoked
on the remote machine. This `<COMMAND>` usually builds but not runs the result.
Example: `./gradlew assembleDebug`, but not `./gradlew installDebug`.

1. Open <kbd>Run</kbd> → <kbd>Edit Configurations...</kbd>
    * To create a new Configuration click the <kbd>+</kbd> button.
    * To change a Configuration Template use the Templates category.
1. Choose a Configuration.
1. Remove default *Build* / *Make* step from the *Before Launch* section using the <kbd>-</kbd> button.
1. Click the <kbd>+</kbd> button.
    * The plugin is available.
        1. Choose the *Mainframer* item.
        1. Insert the `<COMMAND>`.
    * The plugin is not available.
        1. Choose the *Run External Tool* item.
        1. Click the <kbd>+</kbd> button.
        1. Name: something meaningful, like `mainframer <COMMAND>`.
        1. Program: `mainframer`.
        1. Arguments: `<COMMAND>`.
        1. Working Directory: <kbd>Insert Macro</kbd> → `$ProjectFileDir$`.
1. Save.
1. Choose the Configuration and Run it.

