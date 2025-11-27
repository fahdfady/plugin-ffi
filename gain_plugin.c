#include "./plugin_api.h"
#include <stdio.h>

void plugin_init() { printf("Init plugin gain"); }

PluginInfo get_plugin_info() {
  PluginInfo info;
  info.name = "Simple Gain";
  info.author = "fahdashour";
  info.version = 1;

  return info;
}

void process_audio(float *buffer, int length) {
  for (int i = 0; i < length; i++) {
    buffer[i] *= 1.5f;
  }
}

void plugin_cleanup() { printf("Cleaned Up"); }
