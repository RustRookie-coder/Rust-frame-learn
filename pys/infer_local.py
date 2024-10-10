import tensorflow as tf
import numpy as np

from tensorflow.keras.preprocessing import image
from tensorflow.keras.applications.resnet50 import (preprocess_input, decode_predictions)

loaded = tf.saved_model.load('pys/resnet50')
infer = loaded.signatures['serving_default']

img = image.load_img("pys/3.png", target_size=(224, 224))
# x = image.img_to_array(img)
# print('>>> ', x.shape)
# x = np.expand.dims(x, axis=0)
# print('>>> ', x.shape)
# x = preprocess_input(x)
# print('>>> ', x.shape)
