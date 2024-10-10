import tensorflow as tf
# ResNet50 图片分类

from tensorflow.keras.applications.resnet50 import (ResNet50,
                                                    decode_predictions,
                                                    preprocess_input)
model = ResNet50(weights='imagenet')
model.save('/Users/jifeiyu/Documents/Rust-frame-learn/pys/resnet50')
