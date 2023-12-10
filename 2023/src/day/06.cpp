#include <cmath>
#include <cstdint>
#include <iostream>

#define NUMBER_OF_RACES 4
struct RaceInfo_t {
  uint32_t time[NUMBER_OF_RACES];
  uint32_t dist[NUMBER_OF_RACES];
};

RaceInfo_t parse_race_info() {
  RaceInfo_t ret;
  std::string buf;
  std::cin >> buf;

  std::cin >> ret.time[0];
  std::cin >> ret.time[1];
  std::cin >> ret.time[2];
  std::cin >> ret.time[3];

  std::cin >> buf;

  std::cin >> ret.dist[0];
  std::cin >> ret.dist[1];
  std::cin >> ret.dist[2];
  std::cin >> ret.dist[3];

  // std::cout << buf;
  return ret;
};

void race_info_debug(RaceInfo_t *ri) {
  printf("RaceInfo: \n");
  printf("> time: %d %d %d %d\n", ri->time[0], ri->time[1], ri->time[2],
         ri->time[3]);
  printf("> dist: %d %d %d %d\n", ri->dist[0], ri->dist[1], ri->dist[2],
         ri->dist[3]);
}

int main(int argc, char **argv) {
  RaceInfo_t ri = parse_race_info();
  // race_info_debug(&ri);

  // 23133440 is too high

  // (time - wait) * wait > dist
  // time * wait - wait ^ 2 > dist
  // 0 = wait^2 - time * wait + dist
  // wait = time + sqrt( time^2 - 4 * time ) / 2
  uint32_t sum = 1;
  for (int i = 0; i < NUMBER_OF_RACES; i++) {
    uint32_t wait = 3;
    uint32_t total = (ri.time[i] - wait) * wait;
    double t = std::sqrt((ri.time[i] - 4) * ri.time[i]);
    double b = (t + ri.time[i]) / 2.0;
    uint32_t up = b + 1;
    uint32_t span = ri.time[i] - 2 * up;
    sum *= span;
  }
  std::cout << sum;

  // TODO
  return 0;
}
