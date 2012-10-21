#include <SDL/SDL.h>
#include <cstdlib>
#include <ctime>
#ifndef ZYGOTE_H
#define ZYGOTE_H



class Zygote
{
    public:
        Zygote(SDL_Surface *zygoteTexture);
        virtual ~Zygote();
        char getSwimRating();
        char getFlyRating();
        char getRunRating();
        char getStrengthRating();
        char getStaminaRating();
        SDL_Rect getPosition();
        SDL_Surface *getTexture();
    protected:
    private:
        SDL_Rect pos;
        SDL_Surface *texture;
        int swimXP;
        int flyXP;
        int runXP;
        int strengthXP;
        int staminaXP;
        char swimRating;
        char flyRating;
        char runRating;
        char strengthRating;
        char staminaRating;
        unsigned char swimLevel;
        unsigned char flyLevel;
        unsigned char runLevel;
        unsigned char strengthLevel;
        unsigned char staminaLevel;

};

#endif // ZYGOTE_H
