from django.contrib import admin
from django.urls import include, path

urlpatterns = [
    path('renkAPI/', include('renkAPI.urls')),
    path('admin/', admin.site.urls),
]