"use client";
import {
  JSXElementConstructor,
  Key,
  PromiseLikeOfReactNode,
  ReactElement,
  ReactNode,
  ReactPortal,
  SetStateAction,
  useState,
} from "react";
import { AccordionItem, Button } from "@nextui-org/react";
import Script from "next/script";
import { Accordion } from "@nextui-org/react";

import {
  Table,
  TableHeader,
  TableColumn,
  TableBody,
  TableRow,
  TableCell,
  getKeyValue,
} from "@nextui-org/react";
import InterpolationChartComponent from "@/components/ApproximationChart";

const InterpolationPage = () => {
  const [solution, setSolution] = useState<any>("");
  const [isOpen, setIsOpen] = useState(false);
  const [error, setError] = useState("");
  const [value, setValue] = useState<string>("");
  const [solutionFile, setSolutionFile] = useState<any>("");
  const [method, setMethod] = useState<number>(0);
  const [functionId, setFunctionId] = useState<number>(0);
  const [point, setPoint] = useState<any>(0);
  const [nodes, setNodes] = useState<any>(-1);
  const [start, setStart] = useState<any>(0);
  const [end, setEnd] = useState<any>(0);
  const [expanded, setExpanded] = useState("");

  const handleOpen = () => {
    setIsOpen(true);
  };

  const handleClose = () => {
    setIsOpen(false);
  };

  const handleSubmitString = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    setSolution({
      err: "",
      interpolated_value: 0,
    });

    const lines = value.trim().split("\n");
    const x = lines[0]?.split(" ").map(Number); // Convert string array to number array
    const y = lines[1]?.split(" ").map(Number); // Convert string array to number array

    // console.log(JSON.stringify({ x, y, method, functionId, point, nodes }));

    try {
      const response = await fetch(
        "http://localhost:8000/interpolation/string",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            x,
            y,
            // method,
            function: functionId,
            point,
            nodes_amount: nodes,
            start: x[0],
            end: x[x.length - 1],
          }),
        }
      );

      const data = await response.json();
      console.log(data.result);

      if (data.error) {
        handleOpen();
        setError(data.error);
        return;
      }

      setSolution(data.result);
      setSolutionFile(data.result);
    } catch (error) {
      console.error(error);
      handleOpen();
      setError(`Error while processing: ${error}`);
    }
  };

  const handleSubmitFile = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);
    const file = formData.get("file") as File;

    if (!file) {
      console.error("No file uploaded");
      return;
    }

    // Read the file content as text
    const fileContent = await file.text();

    // Split the content by new lines and further split by commas to parse numbers
    const lines = fileContent.split("\n").filter((line) => line.trim() !== "");
    const data = lines.map((line) => line.split(",").map(Number));

    // Assuming the structure is known and fixed as described
    let fileX: any = [],
      fileY: any = [],
      filePoint = null;
    if (data.length > 0 && data[0].length > 0) {
      fileX = data[0];
    }
    if (data.length > 1 && data[1].length > 0) {
      fileY = data[1];
    }
    if (data.length > 2 && data[2].length === 1) {
      filePoint = data[2][0];
    }
    
    try {
      const response = await fetch(
        "http://localhost:8000/interpolation/string",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            x: fileX,
            y: fileY,
            // method,
            function: functionId,
            point: filePoint,
            nodes_amount: nodes,
            start: fileX[0],
            end: fileX[fileX.length - 1],
          }),
        }
      );

      const data = await response.json();
      console.log(data.result);

      if (data.error) {
        handleOpen();
        setError(data.error);
        return;
      }

      setSolution(data.result);
      setSolutionFile(data.result);
    } catch (error) {
      console.error(error);
      handleOpen();
      setError(`Error while processing: ${error}`);
    }
  };

  const renderTable = (tableData, colName) => {
    // Calculate the maximum number of columns from all rows to ensure consistent table structure
    const maxColumns = tableData.reduce(
      (max, row) => Math.max(max, row.length),
      0
    );
    const columns = Array.from(
      { length: maxColumns },
      (_, index) => `${colName} ${index + 1}`
    );

    return (
      <Table aria-label="Dynamic Data Table">
        <TableHeader>
          {columns.map((column, idx) => (
            <TableColumn key={idx}>{column}</TableColumn>
          ))}
        </TableHeader>
        <TableBody>
          {tableData.map((row, idx) => (
            <TableRow key={idx}>
              {row.map((item, index) => (
                <TableCell key={index}>
                  {typeof item === "number" ? item.toFixed(4) : item}
                </TableCell>
              ))}
              {/* Fill in the missing cells if the row has fewer cells than the max */}
              {Array.from({ length: maxColumns - row.length }).map(
                (_, index) => (
                  <TableCell key={`filler-${index}`}></TableCell>
                )
              )}
            </TableRow>
          ))}
        </TableBody>
      </Table>
    );
  };

  const renderDataAccordion = (data) => {
    return Object.entries(data).map(([key, value]) => (
      <AccordionItem key={key} title={key.toUpperCase()} className="mt-2">
        <div className="py-2 px-2 w-full rounded-md text-gray-900 sm:text-sm sm:leading-6">
          <h3>Interpolated Value:</h3>
          <div className="py-2 px-2 w-full rounded-md bg-gray-100 text-gray-900 sm:text-sm sm:leading-6">
            <input
              type="text"
              name="standardDeviation"
              id="standardDeviation"
              value={value.interpolated_value}
              disabled
              className="px-2 block w-50 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            />
          </div>
          {value.difference_table && (
            <div>
              <h4>Difference Table:</h4>
              {renderTable(
                value.difference_table,
                // Array.from(
                //   { length: value.difference_table[0].length },
                //   (_, i) => `Column ${i + 1}`
                // )
                "Delta"
              )}
            </div>
          )}
          {value.nodes && (
            <div className="mt-2">
              <h4>Nodes:</h4>
              {renderTable(value.nodes, ["X", "Y Values"])}
            </div>
          )}
        </div>
      </AccordionItem>
    ));
  };

  return (
    <>
      <Script
        src="https://www.desmos.com/api/v1.8/calculator.js?apiKey=dcb31709b452b1cf9dc26972add0fda6"
        onError={(e: Error) => {
          console.error("Script failed to load", e);
        }}
      />
      <div className="container mx-auto space-y-12 py-8">
        <div className="border-gray-900/10 border-b-2 pb-12">
          <h1 className="text-3xl font-semibold leading-7 text-gray-900">
            Лабораторная работа
          </h1>
          <p className="mt-1 text-md leading-6 text-gray-600 mb-6">
            «Аппроксимация функции методом наименьших квадратов»
          </p>

          <div className="col-span-full mb-6">
            <label className="block text-md font-medium leading-6 text-gray-900">
              Ручной ввод параметров
            </label>
            <div className="mt-2">
              <form onSubmit={handleSubmitString}>
                <textarea
                  name="data"
                  placeholder="
                1.2 2.9 4.1 5.5 6.7 7.8 9.2 10.3\n
                7.4 9.5 11.1 12.9 14.6 17.3 18.2 20.7
                "
                  rows={3}
                  value={value}
                  onChange={(e) => setValue(e.target.value)}
                  className="block w-full rounded-md border-0 px-2 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
                <label className="block mt-2 text-md font-medium leading-6 text-gray-900">
                  Точка для вычисления значения функции
                </label>
                <input
                  className="mt-2 block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                  type="number"
                  value={point}
                  onChange={(e) => setPoint(Number(e.target.value))}
                />
                <div className="mt-2 flex items-center justify-end gap-x-6">
                  <button
                    type="submit"
                    className="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                  >
                    Рассчитать
                  </button>
                </div>
              </form>
            </div>
          </div>

          {isOpen && (
            <div className="fixed inset-0 flex items-center justify-center z-50">
              <div className="bg-gray-800 bg-opacity-75 absolute inset-0"></div>
              <div className="relative bg-white p-8 rounded-lg shadow-lg">
                <button
                  className="absolute top-0 right-0  m-2"
                  onClick={handleClose}
                >
                  <svg
                    className="w-4 h-4"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth="2"
                      d="M6 18L18 6M6 6l12 12"
                    ></path>
                  </svg>
                </button>
                <p>{error}</p>
              </div>
            </div>
          )}

          <div className="col-span-full mb-6">
            <label className="block text-md font-medium leading-6 text-gray-900">
              Ввод параметров из файла
            </label>
            <div className="mt-2">
              <form onSubmit={handleSubmitFile}>
                <input
                  name="file"
                  type="file"
                  className="block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
                <div className="mt-2 flex items-center justify-end gap-x-6">
                  <Button
                    type="submit"
                    className="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                  >
                    Рассчитать
                  </Button>
                </div>
              </form>
            </div>
          </div>

          <div className="response-data border-t border-gray-900/10 pb-12">
            <h1 className="text-2xl font-semibold leading-7 text-gray-900 mt-5">
              Решение
            </h1>

            <div>
              <Accordion variant="bordered" className="mt-2">
                {solution ? (
                  renderDataAccordion(solution)
                ) : (
                  <AccordionItem title="No Data Available">
                    <p>
                      Please interpolate something.
                    </p>
                  </AccordionItem>
                )}
              </Accordion>
            </div>
          </div>

          <div id="gd" className="sm:col-span-6 col-span-1"></div>
          {/* <InterpolationChartComponent solution={solution} /> */}
        </div>
      </div>
    </>
  );
};

export default InterpolationPage;
