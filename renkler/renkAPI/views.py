from django.http import HttpResponse, JsonResponse
import random
from rest_framework.decorators import api_view


COLOURS = [{'White':'#FFFFFF'}, {'Silver':'#C0C0C0'}, {'Gray':'#808080'}, {'Black':'#000000'}]
CORRECT_COLOR = ''

@api_view(['GET'])
def chose_color(request):
    CORRECT_COLOR =random.choice(COLOURS)
    return JsonResponse({'chosen_color':CORRECT_COLOR})

@api_view(['POST', 'GET'])
def index(request):
    if request.method == 'POST':
        color = request.POST.get('color')
        if (color == CORRECT_COLOR):
            return JsonResponse({'result':'correct'})
        else:
            return JsonResponse({'result':'incorrect'})
    else: 
        return JsonResponse({'color':random.choice(COLOURS)})
