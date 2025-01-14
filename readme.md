# Face detection using YoloV8, Tract, Wasm, and Spin

This is an experiment using YoloV8 and Sonos Tract to build a Wasm component
that can performa face detection.

### Testing locally


```
$ Serving http://127.0.0.1:3000
Available Routes:
  face-detection: http://127.0.0.1:3000 (wildcard)
bboxes: [DetectionResult { x1: 197.614, y1: 126.309425, x2: 336.79782, y2: 334.50055, confidence: 0.82691103 }, DetectionResult { x1: 197.25089, y1: 125.750244, x2: 336.60135, y2: 334.41266, confidence: 0.8319721 }, DetectionResult { x1: 195.60527, y1: 125.60892, x2: 336.5612, y2: 334.30304, confidence: 0.81650746 }, DetectionResult { x1: 197.65733, y1: 126.0037, x2: 336.09647, y2: 334.98657, confidence: 0.8404092 }, DetectionResult { x1: 197.4544, y1: 125.86563, x2: 336.068, y2: 334.79312, confidence: 0.8379847 }, DetectionResult { x1: 196.84917, y1: 125.671104, x2: 336.10117, y2: 334.97458, confidence: 0.823186 }, DetectionResult { x1: 198.54927, y1: 125.20853, x2: 335.556, y2: 334.72964, confidence: 0.8399336 }, DetectionResult { x1: 198.45609, y1: 124.89795, x2: 335.74756, y2: 334.60718, confidence: 0.842777 }, DetectionResult { x1: 198.0572, y1: 124.77656, x2: 335.64407, y2: 334.31805, confidence: 0.82766354 }, DetectionResult { x1: 199.0263, y1: 125.022896, x2: 335.3071, y2: 334.76987, confidence: 0.84520483 }]
[DetectionResult { x1: 195.60527, y1: 125.60892, x2: 336.5612, y2: 334.30304, confidence: 0.81650746 }]
```