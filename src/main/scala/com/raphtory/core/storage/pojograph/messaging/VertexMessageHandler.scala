package com.raphtory.core.storage.pojograph.messaging

import java.util.concurrent.atomic.AtomicInteger
import com.raphtory.core.components.querymanager.VertexMessageBatch
import com.raphtory.core.components.querymanager.VertexMessage
import com.raphtory.core.components.querymanager.VertexMessageBatch
import com.raphtory.core.storage.pojograph.PojoGraphLens
import com.raphtory.serialisers.PulsarKryoSerialiser
import com.typesafe.config.Config
import com.typesafe.scalalogging.Logger
import org.apache.pulsar.client.api.Producer
import org.slf4j.LoggerFactory

import scala.collection.mutable

/** @DoNotDocument */
class VertexMessageHandler(
    config: Config,
    producers: Map[Int, Producer[Array[Byte]]],
    pojoGraphLens: PojoGraphLens,
    sentMessages: AtomicInteger,
    receivedMessages: AtomicInteger
) {
  private val kryo: PulsarKryoSerialiser = PulsarKryoSerialiser()

  val logger: Logger = Logger(LoggerFactory.getLogger(this.getClass))

  val msgBatchPath: String  = "raphtory.partitions.batchMessages"
  val messageBatch: Boolean = config.getBoolean(msgBatchPath)
  val maxBatchSize: Int     = config.getInt("raphtory.partitions.maxMessageBatchSize")

  if (messageBatch)
    logger.debug(
            s"Message batching is set to on. To change this modify '$msgBatchPath' in the application conf."
    )

  val totalPartitions: Int = config.getInt("raphtory.partitions.countPerServer") * config.getInt(
          "raphtory.partitions.serverCount"
  )
  logger.debug(s"Setting total partitions to '$totalPartitions'.")

  private val messageCache =
    mutable.Map[Producer[Array[Byte]], mutable.ArrayBuffer[VertexMessage[Any]]]()
  refreshBuffers()

  def sendMessage[T](message: VertexMessage[T]): Unit = {
    sentMessages.incrementAndGet()
    val destinationPartition = (message.vertexId.abs % totalPartitions).toInt
    if (destinationPartition == pojoGraphLens.partitionID) { //sending to this partition
      pojoGraphLens.receiveMessage(message)
      receivedMessages.incrementAndGet()
    }
    else { //sending to a remote partition
      val producer = producers(destinationPartition)
      if (messageBatch) {
        val cache = messageCache(producer)
        cache += message
        if (cache.size > maxBatchSize)
          sendCached(producer)
      }
      else
        producer sendAsync (kryo.serialise(message))
    }

  }

  def sendCached(readerJobWorker: Producer[Array[Byte]]): Unit = {
    readerJobWorker sendAsync kryo.serialise(
            VertexMessageBatch(messageCache(readerJobWorker).toArray)
    )
    messageCache.put(readerJobWorker, mutable.ArrayBuffer[VertexMessage[Any]]())
  }

  def flushMessages(): Unit = {
    logger.debug("Flushing messages in vertex handler.")

    if (messageBatch)
      messageCache.keys.foreach(producer => sendCached(producer))
  }

  def getCountandReset(): Int = {
    logger.debug("Returning count and resetting the message count.")

    sentMessages.getAndSet(0)
  }

  def getCount(): Int =
    sentMessages.get()

  private def refreshBuffers(): Unit = {
    logger.debug("Refreshing messageCache buffers for all Producers.")

    producers.foreach {
      case (key, producer) => messageCache.put(producer, mutable.ArrayBuffer[VertexMessage[Any]]())
    }
  }

}

object VertexMessageHandler {

  def apply(
      config: Config,
      producers: Map[Int, Producer[Array[Byte]]],
      pojoGraphLens: PojoGraphLens,
      sentMessages: AtomicInteger,
      receivedMessages: AtomicInteger
  ) =
    new VertexMessageHandler(
            config: Config,
            producers,
            pojoGraphLens,
            sentMessages,
            receivedMessages
    )
}
