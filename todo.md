## Debug:

## done:
- refactor get window dimension,that is a basic function
- implementing vec rect,pos object to make operation easier,
- for now I am making Position clamp such that the display don't get out of bound.
    - If the bound shape become relatively smaller if display is larger than pixel.
- only render things on the screen
- change the move set of the keyboard
- Now I think if I am using delta x and y,we can get that from function,
Or make a function that will return delta x and delta y,that logic can be moved else where.
- adding black indicator to now what what function I can color.
- fix zooming
- color the pixel somewhat greyish
- import betset and have a 2d vector with it,
- VIP:change the undo button from u to shift U,everytime I press enter,I go to that.
- initialize the canvas to start with alpha value.
- Having a color overlay.
- alpha value having the ability to draw over other value.
## Working
- having a over pop menu that will give list of tools to use
## Todo:
- Moving the canvas implementation in the server side of things.
    - having prelude to be shareble by everyone.
- saving the state of the drawing with each of the uuid.
- all the app state will be saved in the hashmap key being the uuid,
- with 0.25 time the webapp will pass the data to the host,and also update drawing to main,making sure the main canvas and server 
    is synchronous

- having smooth scrolling
- As for simplicity use axum and every time it loads entire bitset from the server and render it
- also code for boundary,canvas not to draw edge of the canvas
    - make position bounded.let not it br any positive value.
- color the background somewhat gradient.



