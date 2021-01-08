const int specialPins[4] = {22, 24, 26, 28};
const int nodePins[8] = {30, 32, 34, 36, 38, 40, 42, 44};
const int notes[7][8] = {
    {24, 26, 27, 29, 31, 33, 35, 36},
    {36, 38, 40, 41, 43, 45, 47, 48},
    {48, 50, 52, 53, 55, 57, 59, 60},
    {60, 62, 64, 65, 67, 69, 71, 72},
    {72, 74, 76, 77, 79, 81, 83, 84},
    {84, 86, 88, 89, 91, 93, 95, 96},
    {96, 98, 100, 101, 103, 105, 107, 108}
};

const int MINREG = 0, MAXREG = 6;

int noteStates[8] = {0, 0, 0, 0, 0, 0, 0, 0};

int reg = 3;
int sharp = 0, flat = 0;

void setup() {
    // put your setup code here, to run once:
    for (int i : specialPins) {
        pinMode(i, INPUT);  // all the buttons are input
    }

    Serial.begin(9600); // open serial port
}

int buttonState;

void loop() {
    // put your main code here, to run repeatedly:
    for (int i=0; i<8; i++) {
        // note button states
        buttonState = digitalRead(nodePins[i]);
        if (buttonState == HIGH) {
            // send "on" for notes[reg][i] + sharp + flat
            if (noteStates[i]) continue;
            Serial.print(notes[reg][i] + sharp + flat);
            Serial.println(" on");
            noteStates[i] = 1;
        } else if (buttonState == LOW) {
            // send "off" for notes[reg][i] + sharp + flat
            if (!noteStates[i]) continue;
            Serial.print(notes[reg][i] + sharp + flat);
            Serial.println(" off");
            noteStates[i] = 0;
        }
    }

    // special button state - sharp
    buttonState = digitalRead(specialPins[0]);
    if (buttonState == HIGH) {
        sharp = 1;
    } else if (buttonState == LOW) {
        sharp = 0;
    }

    // special button state - flat
    buttonState = digitalRead(specialPins[1]);
    if (buttonState == HIGH) {
        flat = 1;
    } else if (buttonState == LOW) {
        flat = 0;
    }

    // special button state - octave up
    buttonState = digitalRead(specialPins[2]);
    if (buttonState == HIGH) {
        reg++;
        reg = min(MAXREG, reg);
    }

    // special button state - octave down
    buttonState = digitalRead(specialPins[3]);
    if (buttonState == HIGH) {
        reg--;
        reg = max(MINREG, reg);
    }
}
