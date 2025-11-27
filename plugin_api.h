#ifndef PLUGIN_API_H
#define PLUGIN_API_H

typedef struct {
  const char *name;
  const char *author;
  int version;
} PluginInfo;

PluginInfo get_plugin_info();

void plugin_init();

void process_audio(float *buffer, int length);

void plugin_cleanup();

#endif
