import cairo
import math
import gi
from numpy import linspace

gi.require_version("Gtk", "3.0")
from gi.repository import Gtk

# ~~edit me~~
#canvas "width" and "height"
width = 400
height = 400
#length of long notch
long_notch = 3
#length of short notch
short_notch = 2.5
#how often notches should show up
notch_interval = 25
#speedometer range (lowest speed, highest speed)
bounds = [0, 100]

#don't edit me
width_center = width / 2
height_center = height / 2
degree_offset = bounds[1] / 6
#for now, just assume number of notches is the highest speed
notch_count = bounds[1]

class MyWindow(Gtk.Window):
    def __init__(self):
        super().__init__(title = "Speedometer")

        self.darea = Gtk.DrawingArea()
        self.darea.connect("draw", self.on_draw)
        self.add(self.darea)

    def on_draw(self, wid, cr):
        #draw circle
        cr.set_source_rgb(255, 255, 255)
        cr.arc((width / 2), (height / 2), (height / 2), 0, (2 * math.pi))
        cr.set_source_rgb(0, 0, 0)
        cr.fill_preserve()
        cr.set_source_rgb(255, 255, 255)

        #draw notches
        for notch_num, i in enumerate(linspace(bounds[0] - degree_offset, bounds[1] + degree_offset, num = notch_count)):
            x, y = get_coordinates_from_speed(i, bounds)
            #set start position along circle
            cr.move_to((x * width_center) + width_center, (y * -height_center) + height_center)
            #set end position based on whether notch should be long
            if (notch_num + 1) % notch_interval == 0 or notch_num == 0: 
                cr.line_to(x * (width / long_notch) + width_center, y * (height / -long_notch) + height_center)
            else:
                cr.line_to(x * (width / short_notch) + width_center, y * (height / -short_notch) + height_center)

        #finalize drawing
        cr.stroke()

#given speed and boundaries (lowest speed at angle pi and highest speed at angle 0), output speedometer coordinates
def get_coordinates_from_speed(speed: int, bounds: tuple[int, int]):
    modifier = bounds[1] / math.pi
    midpoint = (bounds[0] + bounds[1]) / 2
    return (math.sin((speed - midpoint) / modifier), math.cos((speed - midpoint) / modifier))

win = MyWindow()
win.connect("destroy", Gtk.main_quit)
win.show_all()
Gtk.main()