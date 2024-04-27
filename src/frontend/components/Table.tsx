import React from 'react';
import { Accordion, AccordionItem, Table, TableHeader, TableColumn, TableBody, TableRow, TableCell } from '@nextui-org/react'; // Ensure correct import paths

interface MethodData {
  [key: string]: Array<[number, number]>;
}

interface Props {
  solution: MethodData | null;
  error: string;
}

const DifferentialSolverResults: React.FC<Props> = ({ solution, error }) => {
  const renderTable = (tableData: any, columns: string[]) => {
    // Add a check to confirm tableData is an array
    if (!Array.isArray(tableData.points)) {
      console.error('Invalid tableData:', tableData.points); // Debug output
      return <p>Data format error</p>; // Provide a fallback UI
    }
    let table = tableData.points;
    return (
      <Table aria-label="Dynamic Data Table">
        <TableHeader>
          {columns.map((column, idx) => (
            <TableColumn key={idx}>{column}</TableColumn>
          ))}
        </TableHeader>
        <TableBody>
          {table.map((row: any[], idx: React.Key | null | undefined) => (
            <TableRow key={idx}>
              {row.map((item, index) => (
                <TableCell key={index}>
                  {typeof item === 'number' ? item.toFixed(4) : item}
                </TableCell>
              ))}
            </TableRow>
          ))}
        </TableBody>
      </Table>
    );
  };

  const renderDataAccordion = (data: MethodData) => (
    Object.entries(data).map(([methodName, values]) => (
      <AccordionItem key={methodName} title={methodName.toUpperCase().replaceAll("EXTENDEDEULER", "EXTENDED EULER")} className="mt-2">
        <div className="py-2 px-2 w-full rounded-md text-gray-900">
          <h3>Results:</h3>
          {renderTable(values, ['x', 'y'])}
        </div>
      </AccordionItem>
    ))
  );

  return (
    <div>
      <Accordion variant="bordered" className="mt-2">
        {solution && error === "" ? (
          renderDataAccordion(solution)
        ) : (
          <AccordionItem title="No Data Available">
            <p>Please interpolate something.</p>
          </AccordionItem>
        )}
      </Accordion>
    </div>
  );
};

export default DifferentialSolverResults;
