#! /bin/python3

class Node:
    def __init__(self, weight_total: int, weight_init: int):
        self.w_total = weight_total
        self.w_init = weight_init
        self.w_curr = 0

    def over(self):
        self.w_curr += self.w_init

    def choose(self):
        self.w_curr -= self.w_total

    def __str__(self):
        return "%d" % self.w_curr


class SmoothWeightRoundRobin:
    def __init__(self, weights: [int]):
        self.nodes = list()
        w_total = sum(weights)
        for w in weights:
            self.nodes.append(Node(w_total, w))

    def __str__(self):
        nodes = list()
        for n in self.nodes:
            nodes.append(n.w_curr)
        return str(nodes)

    def run(self):
        while True:
            idx = -1
            first = str(self)
            for i, n in enumerate(self.nodes):
                n.over()
                if idx == -1:
                    idx = i
                elif self.nodes[i].w_curr > self.nodes[idx].w_curr:
                    idx = i
            before = str(self)
            self.nodes[idx].choose()
            after = str(self)
            print("%16s-> %16s -> %16s" % (first, before, after))


if __name__ == "__main__":
    b = SmoothWeightRoundRobin([1, 2, 3, 4])
    b.run()
