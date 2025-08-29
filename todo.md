## done:
- refactor get window dimension,that is a basic function
- implementing vec rect,pos object to make operation easier,
- for now I am making Position clamp such that the display don't get out of bound.
    - If the bound shape become relatively smaller if display is larger than pixel.
- only render things on the screen
    - As grid size is constant,let's make everything grid related constant.
- change the move set of the keyboard
- Now I think if I am using delta x and y,we can get that from function,
Or make a function that will return delta x and delta y,that logic can be moved else where.
- adding black indicator to now what what function I can color.
- fix zooming
- color the pixel somewhat greyish
- import betset and have a 2d vector with it,
- VIP:change the undo button from u to shift U,everytime I press enter,I go to that.
## Working
- initialize the canvas to start with alpha value.
- Having a color overlay.
- alpha value having the ability to draw over other value.
## Todo:
- having a over pop menu that will give list of tools to use
- having smooth scrolling
- As for simplicity use axum and every time it loads entire bitset from the server and render it
    Input from the browser will pass as lineDraw,
- also code for boundary,canvas not to draw edge of the canvas
    - make position bounded.let not it br any positive value.
- color the background somewhat gradient.



