from pyraphtory.api.algorithm import PyAlgorithm
from pyraphtory.api.graph import TemporalGraph
from pyraphtory.api.vertex import Vertex
from pyraphtory.api.scala.implicits.numeric import Long


class LocalTriangleCount(PyAlgorithm):
    def __call__(self, graph: TemporalGraph) -> TemporalGraph:
        def step1(v: Vertex):
            v["triangleCount"] = 0
            neighbours = {n: False for n in v.neighbours()}
            v.message_all_neighbours(neighbours)

        def step2(v: Vertex):
            neighbours = v.neighbours()
            queue = v.message_queue()
            tri = 0
            for msg in queue:
                tri += len(set(neighbours).intersection(msg.keys()))
            v['triangleCount'] = int(tri / 2)

        return graph.step(step1).step(step2)

    def tabularise(self, graph: TemporalGraph):
        return graph.step(lambda v: v.set_state("name",v.name())).step(lambda v: v.set_state("triangleCount",v['triangleCount'])).select("name", "triangleCount")


class GlobalTriangleCount(LocalTriangleCount):
    def __call__(self, graph: TemporalGraph) -> TemporalGraph:
        return (super().__call__(graph)
                .set_global_state(lambda s: s.new_adder[Long](name="triangles", initial_value=0, retain_state=True))
                .step(lambda v, s: s["triangles"].add(v["triangleCount"]))
                .set_global_state(lambda s: s.new_constant("triangleCountResult", s["triangles"].value() // 3)))

    def tabularise(self, graph: TemporalGraph):
        return graph.global_select("triangleCountResult")