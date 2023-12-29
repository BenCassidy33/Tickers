import StockDropDown from './components/stock-panel';
import RangeCalendar from './components/range-calendar';

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from '@/components/ui/resizable';

export default function App() {
  return (
    <div className="w-full h-screen p-0 m-0 bg-black">
      <ResizablePanelGroup direction="vertical" className="rounded-lg border">
        <ResizablePanel defaultSize={60}>
          <ResizablePanelGroup direction="horizontal">
            <ResizablePanel defaultSize={50}>
              <div className="flex h-[200px] items-center justify-center p-6">
                <StockDropDown />
                <div className="px-5"></div>
                <RangeCalendar />
              </div>
            </ResizablePanel>
            <ResizableHandle className="bg-gray-100" />
            <ResizablePanel defaultSize={50}>
              <div className="flex h-full items-center justify-center p-6">
                <span className="font-semibold text-white">
                  This is where the Table will go
                </span>
              </div>
            </ResizablePanel>
          </ResizablePanelGroup>
        </ResizablePanel>

        <ResizableHandle className="bg-gray-500" />
        <ResizablePanel defaultSize={40}>
          <span className="font-semibold text-white">
            This is where the bar graph will go
          </span>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}
