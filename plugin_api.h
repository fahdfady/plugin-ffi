#ifndef PLUGIN_API_H
#define PLUGIN_API_H

// metadata
typedef struct {
  const char *name;
  const char *author;
  int version;
} PluginInfo;

void plugin_init();

PluginInfo get_plugin_info();

void process_audio(float *buffer, int length);

void plugin_cleanup();

#endif
