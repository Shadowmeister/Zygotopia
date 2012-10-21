#include "../include/Zygote.h"

Zygote::Zygote(SDL_Surface *zygoteTexture)
{
    this->pos.x = 16;
    this->pos.y = 16;
    this->texture = zygoteTexture;
    this->texture.format.
    this->swimXP = 0;
    this->flyXP = 0;
    this->runXP = 0;
    this->strengthXP = 0;
    this->staminaXP = 0;
    this->swimLevel = 0;
    this->flyLevel = 0;
    this->runLevel = 0;
    this->strengthLevel = 0;
    this->staminaLevel = 0;
    srand(time(0));
    int num = (int)(rand()*3 + 0.5f);
    switch(num)
    {
        case 0:
            this->swimRating = 'D';
            break;
        case 1:
            this->swimRating = 'C';
            break;
        case 2:
            this->swimRating = 'B';
            break;
        case 3:
            this->swimRating = 'A';
            break;
    }
    num = (int)(rand()*3 + 0.5f);
    switch(num)
    {
        case 0:
            this->flyRating = 'D';
            break;
        case 1:
            this->flyRating = 'C';
            break;
        case 2:
            this->flyRating = 'B';
            break;
        case 3:
            this->flyRating = 'A';
            break;
    }
    num = (int)(rand()*3 + 0.5f);
    switch(num)
    {
        case 0:
            this->runRating = 'D';
            break;
        case 1:
            this->runRating = 'C';
            break;
        case 2:
            this->runRating = 'B';
            break;
        case 3:
            this->runRating = 'A';
            break;
    }
    num = (int)(rand()*3 + 0.5f);
    switch(num)
    {
        case 0:
            this->strengthRating = 'D';
            break;
        case 1:
            this->strengthRating = 'C';
            break;
        case 2:
            this->strengthRating = 'B';
            break;
        case 3:
            this->strengthRating = 'A';
            break;
    }
    num = (int)(rand()*3 + 0.5f);
    switch(num)
    {
        case 0:
            this->staminaRating = 'D';
            break;
        case 1:
            this->staminaRating = 'C';
            break;
        case 2:
            this->staminaRating = 'B';
            break;
        case 3:
            this->staminaRating = 'A';
            break;
    }

}

Zygote::~Zygote()
{
    //dtor
}

char Zygote::getSwimRating()
{
    return this->swimRating;
}

char Zygote::getFlyRating()
{
    return this->flyRating;
}

char Zygote::getRunRating()
{
    return this->runRating;
}

char Zygote::getStrengthRating()
{
    return this->strengthRating;
}

char Zygote::getStaminaRating()
{
    return this->staminaRating;
}

SDL_Rect Zygote::getPosition()
{
    return this->pos;
}

SDL_Surface *Zygote::getTexture()
{
    return this->texture;
}
