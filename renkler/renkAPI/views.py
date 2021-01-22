from django.http import HttpResponse, JsonResponse
import random, json
from rest_framework.decorators import api_view


COLORS = [{'color':'White', 'hex':'#FFFFFF'}, 
            {'color':'Silver', 'hex':'#C0C0C0'}, 
            {'color':'Gray', 'hex':'#808080'}, 
            {'color':'Black', 'hex':'#000000'}]


@api_view(['GET'])
def chose_color(request):
    request.session['CORRECT_COLOR'] =random.choice(COLORS)['color']
    return JsonResponse({'chosen_color':request.session['CORRECT_COLOR']})

@api_view(['POST', 'GET'])
def index(request):
    if request.method == 'POST':
        body_unicode = request.body.decode('utf-8')
        body = json.loads(body_unicode)
        color = body['color']
        # color = request.POST.get('color')['color']
        if (color == request.session.get('CORRECT_COLOR')):
            return JsonResponse({'result':'correct'})
        else:
            return JsonResponse({'result':'incorrect'})
    else: 
        return JsonResponse({'color':random.choice(COLORS)})
