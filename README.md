# PassPass
A small cli tool to take the password from the clipboard and type it into an application that doesn't support pasting.

## Default behaviour
PassPass will take whatever is on the clipboard and type it out for you after a 3 second delay.

After typing the password PassPass will clear the clipboard

### Extra Options
Currently only 2 options are supported as a test, and could be expanded in the future when more features are added.

`--no-clear` Will disable clearing the clipboard

`--confirm` Will hit Return/Enter for you after typing the password


## Security
I am no security expert but I cannot think of any security vulnerabilities with this tool that don't already exist in the first place by copying the password into the clipboard
