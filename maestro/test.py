import maestro
import time

servo = maestro.Controller()
for s in range(0,6):
    servo.setSpeed(s, 0)
    servo.setRange(s, 2000, 10000)
while True:
    for s in range(0,6):
        servo.setTarget(s, 2000)
    time.sleep(2)
    for s in range(0,6):
        servo.setTarget(s, 6000)
    time.sleep(2)
    for s in range(0,6):
        servo.setTarget(s, 10000)
    time.sleep(2)
servo.close()