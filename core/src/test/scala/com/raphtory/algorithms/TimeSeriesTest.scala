package com.raphtory.algorithms

import com.raphtory.BaseCorrectnessTest
import com.raphtory.TimeSeriesGraphState

class TimeSeriesTest extends BaseCorrectnessTest {
  test("Time Series Test") {
    assert(
            correctnessTest(
                    TimeSeriesGraphState(),
                    "TimeSeriesGraphState/timeSeries.csv",
                    "TimeSeriesGraphState/timeSeriesResult.csv",
                    4
            )
    )
  }
}
