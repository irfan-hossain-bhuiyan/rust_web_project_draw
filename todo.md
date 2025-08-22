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
## Working:
## Todo:
- VIP:change the undo button from u to shift U,everytime I press enter,I go to that.
- also code for boundary,canvas not to draw edge of the canvas
    - make position bounded.let not it br any positive value.
- color the pixel somewhat greyish
- having smooth scrolling

