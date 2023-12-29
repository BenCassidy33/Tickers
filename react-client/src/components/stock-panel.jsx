import * as React from 'react';

import { CheckIcon, ChevronsUpDown } from 'lucide-react';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from '@/components/ui/command';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';

const tickers = [
  {
    label: 'Apple (AAPL)',
    value: 'Apple (AAPL)',
    ticker_value: 'AAPL',
  },
  {
    label: 'Microsoft (MSFT)',
    value: 'Microsoft (MSFT)',
    ticker_value: 'MSFT',
  },
  {
    label: 'Google (GOOGL)',
    value: 'Google (GOOGL)',
    ticker_value: 'GOOGL',
  },
  {
    label: 'Amazon (AMZN)',
    value: 'Amazon (AMZN)',
    ticker_value: 'AMZN',
  },
  {
    label: 'Tesla (TSLA)',
    value: 'Tesla (TSLA)',
    ticker_value: 'TSLA',
  },
  {
    label: 'J.P. Morgan (JPM)',
    value: 'J.P. Morgan (JPM)',
    ticker_value: 'JPM',
  },
  {
    label: 'Visa (V)',
    value: 'Visa (V)',
    ticker_value: 'V',
  },
  {
    label: 'PayPal (PYPL)',
    value: 'PayPal (PYPL)',
    ticker_value: 'PYPL',
  },
  {
    label: 'Netflix (NFLX)',
    value: 'Netflix (NFLX)',
    ticker_value: 'NFLX',
  },
  {
    label: 'Goldman Sachs (GS)',
    value: 'Goldman Sachs (GS)',
    ticker_value: 'GS',
  },
  {
    label: 'Walt Disney (DIS)',
    value: 'Walt Disney (DIS)',
    ticker_value: 'DIS',
  },
  {
    label: 'IBM (IBM)',
    value: 'IBM (IBM)',
    ticker_value: 'IBM',
  },
  {
    label: 'Boeing (BA)',
    value: 'Boeing (BA)',
    ticker_value: 'BA',
  },
  {
    label: 'General Electric (GE)',
    value: 'General Electric (GE)',
    ticker_value: 'GE',
  },
];

function renderPretty(ticker) {
  let val = ticker.split(' ');
  return (
    <span className="flex">
      <span>{val[0]} </span>
      <div className="px-[2px]"></div>
      <span className="uppercase">{val[1]}</span>
    </span>
  );
}

export default function StockDropDown() {
  const [open, setOpen] = React.useState(false);
  const [value, setValue] = React.useState('');

  return (
    <Popover open={open} onOpenChange={setOpen} className="rounded-s">
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-[200px] justify-between text-white"
        >
          {value ? (
            <span>
              <span className="capitalize">{renderPretty(value)}</span>
            </span>
          ) : (
            'Select ticker...'
          )}
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[200px] p-0">
        <Command>
          <CommandInput placeholder="Search ticker..." className="h-9" />
          <CommandEmpty>No ticker found.</CommandEmpty>
          <CommandGroup>
            {tickers.map((ticker) => (
              <CommandItem
                key={ticker.value}
                value={ticker.value}
                onSelect={(currentValue) => {
                  setValue(currentValue);
                  setOpen(false);
                }}
              >
                {ticker.label}
                <CheckIcon
                  className={cn(
                    'ml-auto h-4 w-4',
                    value === ticker.value ? 'opacity-100' : 'opacity-0',
                  )}
                />
              </CommandItem>
            ))}
          </CommandGroup>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
