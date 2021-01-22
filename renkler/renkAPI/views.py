from django.http import HttpResponse, JsonResponse
import random

COLOURS = ['#FFFFFF', '#C0C0C0', '#808080', '#000000']

def index(request):
    return JsonResponse({'color':random.choice(COLOURS)})