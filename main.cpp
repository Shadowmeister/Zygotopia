#include "include/Zygote.h"
#include <string>
#include <iostream>

//The attributes of the screen
const int SCREEN_WIDTH = 640;
const int SCREEN_HEIGHT = 640;
const int SCREEN_BBP = 32;

// The sufaces that will be used
SDL_Surface *grass = NULL;
SDL_Surface *water = NULL;
SDL_Surface *screen = NULL;
SDL_Surface *zygoteTexture = NULL;

// the event struction that will be used
SDL_Event event;

// function prototypes
SDL_Surface *load_image( std::string filename);
void apply_surface( int x, int y, SDL_Surface* source, SDL_Surface* destination);
bool init();
void cleanUp();
bool update();

enum tileTypes {grassT, waterT};

int world[32][32];

void generateWorld(){
    for(int i = 0; i < 32; i++)
    {
        for(int j = 0; j < 32; j++)
        {
            if((i < (29 - j)) && (i > (3+j)) && j < 6)
            {
                world[i][j] = waterT;
            } else
            {
                world[i][j] = grassT;
            }
        }
    }
}

Zygote *zygote = new Zygote(zygoteTexture);
int main(int argc, char** argv)
{
    // ready to quit?
    bool quit = false;
    if(init() == false)
    {
        return 1;
    }

    //Update the screen
    while( quit == false)
    {
        while( SDL_PollEvent( &event))
        {
            if( event.type == SDL_QUIT)
            {
                //Quit the program
                quit = true;
            }
        }
       if(update() == false)
       {
           return 1;
       }
    }


    cleanUp();

    // Return
    return 0;
}

bool update()
{
    //Apply the grass to the screen
    for(int i = 0; i < 32; i++)
    {
        for(int j = 0; j < 32; j++)
        {
            if(world[i][j] == grassT)
            {
                apply_surface(i*20, j*20, grass, screen);
            } else if(world[i][j] == waterT)
            {
                apply_surface(i*20, j*20, water, screen);
            }

        }
    }

    apply_surface(zygote->getPosition().x*20, zygote->getPosition().y*20, zygoteTexture, screen);

    if(SDL_Flip(screen) == -1)
    {
        return false;
    }

    return true;
}

bool init()
{
    //  Initialize all SDL subsystems
    if( SDL_Init(SDL_INIT_EVERYTHING) == -1)
    {
        return false;
    }

    // Set up the screen
    screen = SDL_SetVideoMode(SCREEN_WIDTH, SCREEN_HEIGHT, SCREEN_BBP, SDL_SWSURFACE);

    // If there was an error in setting up the screen
    if( screen == NULL)
    {
        return false;
    }

    // Set the window caption
    SDL_WM_SetCaption("Zygotopia", NULL);

    // Load the images
    grass = load_image("../../pics/grass.bmp");
    water = load_image("../../pics/water.bmp");
    zygoteTexture = load_image("../../pics/zygote.bmp");

    // check for errors loading images
    if(grass == NULL)
    {
        std::cout << "image for grass not found" << std::endl;
        return false;
    }
    if(water == NULL)
    {
        std::cout << "image for water not found" << std::endl;
        return false;
    }
    if(zygoteTexture == NULL)
    {
        std::cout << "image for zygote not found" << std::endl;
        return false;
    }

    generateWorld();

    return true;
}

void cleanUp()
{
    //Free the surfaces
    SDL_FreeSurface( grass);
    SDL_FreeSurface( water);
    SDL_FreeSurface( zygoteTexture);

    //Quit SDL
    SDL_Quit();
}

SDL_Surface *load_image( std::string filename)
{
    // Temporary storage for the image that's loaded
    SDL_Surface* loadedImage = NULL;

    // the optimized image that will be used
    SDL_Surface* optimizedImage = NULL;

    // Load the image
    loadedImage = SDL_LoadBMP( filename.c_str() );

    if(loadedImage != NULL)
    {
        // Create an optimized image
        optimizedImage = SDL_DisplayFormat( loadedImage );

        //Free the old image
        SDL_FreeSurface (loadedImage);

        //If the image was optimized fine
        if( optimizedImage != NULL)
        {
            // Map the color key
            Uint32 colorKey = SDL_MapRGB( optimizedImage->format, 0xFF, 0x7F, 0xFF);

            //Set all pixels of color r 0xFF, g 0x7F, b 0xFF to be transparent
            SDL_SetColorKey(optimizedImage, SDL_SRCCOLORKEY, colorKey);
        }
    }
    // Return the optimized image
    return optimizedImage;
}

void apply_surface(int x, int y, SDL_Surface* source, SDL_Surface* destination)
{
    // Make a temporary rectangle to hold the offsets
    SDL_Rect offset;

    // Give the offsets the the rectangle
    offset.x = x;
    offset.y = y;

    // Blit the surface
    SDL_BlitSurface(source, NULL, destination, &offset);
}
