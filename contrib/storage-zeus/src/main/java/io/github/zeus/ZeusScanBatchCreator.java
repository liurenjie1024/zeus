package io.github.zeus;

import org.apache.drill.common.exceptions.ExecutionSetupException;
import org.apache.drill.exec.ops.FragmentContext;
import org.apache.drill.exec.physical.impl.BatchCreator;
import org.apache.drill.exec.physical.impl.ScanBatch;
import org.apache.drill.exec.record.CloseableRecordBatch;
import org.apache.drill.exec.record.RecordBatch;
import org.apache.drill.exec.store.RecordReader;

import java.util.Collections;
import java.util.List;

/**
 * Created by liurenjie on 25/01/2018.
 */
public class ZeusScanBatchCreator implements BatchCreator<ZeusSubScan> {
  @Override
  public CloseableRecordBatch getBatch(FragmentContext context, ZeusSubScan subScan,
                                       List<RecordBatch> children) throws ExecutionSetupException {
    ZeusRecordReader recordReader = new ZeusRecordReader(subScan.getPlugin().getClient(),
      subScan, subScan.getColumns());
    return new ScanBatch(subScan, context, Collections.singletonList(recordReader));
  }
}
