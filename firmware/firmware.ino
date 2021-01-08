const int specialPins[4] = {24, 22, 26, 28};
const int nodePins[8] = {30, 32, 33, 36, 39, 40, 42, 44};
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

int noteStates[200];
int octaveStates[2] = {0, 0};   // up, down
int modStates[2] = {0, 0};  // sharp, flat

int sharp = 0, flat = 0;

int reg = 3;

void clearReg() {
    for (int i=0; i<200; i++) {
        if (!noteStates[i]) continue;
        noteStates[i] = 0;
        Serial.print(i);
        Serial.println(" off");
    }
}

void setup() {
    // put your setup code here, to run once:
    Serial.begin(9600); // open serial port
    
    for (int i : specialPins) {
        pinMode(i, INPUT_PULLUP);  // all the buttons are input
    }

    for (int i : nodePins) {
        pinMode(i, INPUT_PULLUP);
    }
}

int buttonState;

void loop() {
    // put your main code here, to run repeatedly:
    for (int i=0; i<8; i++) {
        // note button states
        buttonState = digitalRead(nodePins[i]);
        int note = notes[reg][i] + sharp - flat;
        if (buttonState == HIGH && !noteStates[note]) {
            // send "on"
            Serial.print(note);
            Serial.println(" on");
            noteStates[note] = 1;
        } else if (buttonState == LOW && noteStates[note]) {
            // send "off"
            Serial.print(note);
            Serial.println(" off");
            noteStates[note] = 0;
        }
    }

    // special button state - sharp
    buttonState = digitalRead(specialPins[0]);
    if (buttonState == HIGH && !modStates[0]) {
        modStates[0] = 1;
        sharp = 1;
        clearReg();
    } else if (buttonState == LOW && modStates[0]) {
        modStates[0] = 0;
        sharp = 0;
        clearReg();
    }

    // special button state - flat
    buttonState = digitalRead(specialPins[1]);
    if (buttonState == HIGH && !modStates[1]) {
        modStates[1] = 1;
        flat = 1;
        clearReg();
    } else if (buttonState == LOW && modStates[1]) {
        modStates[1] = 0;
        flat = 0;
        clearReg();
    }

    // special button state - octave up
    buttonState = digitalRead(specialPins[2]);
    if (buttonState == HIGH && !octaveStates[0]) {
        octaveStates[0] = 1;
        reg++;
        reg = min(MAXREG, reg);
        clearReg();
    } else if (buttonState == LOW && octaveStates[0]) {
        octaveStates[0] = 0;
        clearReg();
    }

    // special button state - octave down
    buttonState = digitalRead(specialPins[3]);
    if (buttonState == HIGH && !octaveStates[1]) {
        octaveStates[1] = 1;
        reg--;
        reg = max(MINREG, reg);
        clearReg();
    } else if (buttonState == LOW && octaveStates[1]) {
        octaveStates[1] = 0;
        clearReg();
    }
    delay(50);
}
