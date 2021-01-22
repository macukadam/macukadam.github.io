from django.http import HttpResponse, JsonResponse
import random
from rest_framework.decorators import api_view


COLORS = [{'color':'White', 'hex':'#FFFFFF'}, 
            {'color':'Silver', 'hex':'#C0C0C0'}, 
            {'color':'Gray', 'hex':'#808080'}, 
            {'color':'Black', 'hex':'#000000'}]

CORRECT_COLOR = ''

@api_view(['GET'])
def chose_color(request):
    CORRECT_COLOR =random.choice(COLORS)['color']
    return JsonResponse({'chosen_color':CORRECT_COLOR})

@api_view(['POST', 'GET'])
def index(request):
    if request.method == 'POST':
        color = request.POST.get('color')['color']
        if (color == CORRECT_COLOR):
            return JsonResponse({'result':'correct'})
        else:
            return JsonResponse({'result':'incorrect'})
    else: 
        return JsonResponse({'color':random.choice(COLORS)})
