from django.urls import path

from . import views

urlpatterns = [
    path('', views.index, name='index'),
    path('chose-color', views.chose_color, name='chose_color'),
]