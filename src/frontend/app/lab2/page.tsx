"use client";
import {
  Chart as ChartJS,
  LinearScale,
  PointElement,
  LineElement,
  Tooltip,
  Legend,
} from "chart.js";
import { Scatter } from "react-chartjs-2";
import { useEffect, useState } from "react";
import {
  Table,
  TableHeader,
  TableColumn,
  TableBody,
  TableRow,
  TableCell,
  getKeyValue,
} from "@nextui-org/react";

const LinearEquationPage = () => {
  const [solution, setSolution] = useState<any>("");
  const [value, setValue] = useState<string>("");
  const [isOpen, setIsOpen] = useState(false);
  const [error, setError] = useState("");
  const [rows, setRows] = useState(2);
  const [halfdivisonTableRows, setHalfdivisonTableRows] = useState<any>([
    {
      key: 0,
      iteration: 0,
      a: 0,
      b: 0,
      x: 0,
      fa: 0,
      fb: 0,
      fx: 0,
      abs_diff: 0,
      err: "",
    },
  ]);
  const [simpleiterationTableRows, setSimpleiterationTableRows] = useState<any>(
    [
      {
        key: 0,
        iteration: 0,
        x_k: 0,
        f_x_k: 0,
        x_k_plus_one: 0,
        phi_x_k: 0,
        abs_diff: 0,
        err: "",
      },
    ]
  );
  const [newtonTableRows, setNewtonTableRows] = useState<any>([
    {
      key: 0,
      iteration: 0,
      x_k: 0,
      f_x_k: 0,
      f_prime_x_k: 0,
      x_k_plus_one: 0,
      abs_diff: 0,
      err: "",
    },
  ]);

  const [formData, setFormData] = useState({
    eq_id: 0,
    interval: [],
    intervalMin: 0,
    intervalMax: 0,
    estimate: 0,
    method_id: 0,
  });

  const handleChange = (e: any) => {
    const { name, value } = e.target;

    if (name === "method_id") {
      setHalfdivisonTableRows([
        {
          key: 0,
          iteration: 0,
          a: 0,
          b: 0,
          x: 0,
          fa: 0,
          fb: 0,
          fx: 0,
          abs_diff: 0,
          err: "",
        },
      ]);

      setSimpleiterationTableRows([
        {
          key: 0,
          iteration: 0,
          x_k: 0,
          f_x_k: 0,
          x_k_plus_one: 0,
          phi_x_k: 0,
          abs_diff: 0,
          err: "",
        },
      ]);

      setNewtonTableRows([
        {
          key: 0,
          iteration: 0,
          x_k: 0,
          f_x_k: 0,
          f_prime_x_k: 0,
          x_k_plus_one: 0,
          abs_diff: 0,
          err: "",
        },
      ]);
    }
    setFormData({
      ...formData,
      [name]: Number(value),
    });
  };

  const options = {
    scales: {
      y: {
        title: {
          display: true,
          text: solution ? "Вектор погрешности" : "Вектор погрешности",
        },
      },
      x: {
        title: {
          display: true,
          text: "Вектор неизвестных х",
        },
      },
    },
    plugins: {
      legend: {
        position: "top" as const,
      },
      title: {
        display: true,
        text: "График функции",
      },
    },
  };

  const data = {
    datasets: [
      {
        label: "Вектора",
        data: [],
        backgroundColor: "rgba(255, 99, 132, 1)",
      },
    ],
  };

  const halfdivisonTableColumns = [
    {
      key: "iteration",
      label: "№ шага",
    },
    {
      key: "a",
      label: "a",
    },
    {
      key: "b",
      label: "b",
    },
    {
      key: "x",
      label: "x",
    },
    {
      key: "fa",
      label: "f(a)",
    },
    {
      key: "fb",
      label: "f(b)",
    },
    {
      key: "fx",
      label: "f(x)",
    },
    {
      key: "abs_diff",
      label: "|a-b|",
    },
  ];

  const simpleiterationTableColumns = [
    {
      label: "№ шага",
      key: "iteration",
    },
    {
      label: "x_k",
      key: "x_k",
    },
    {
      label: "f(x_k)",
      key: "f_x_k",
    },
    {
      label: "x_k+1",
      key: "x_k_plus_one",
    },
    {
      label: "phi(x_k)",
      key: "phi_x_k",
    },
    {
      label: "|x_k-x_{k+1}|",
      key: "abs_diff",
    },
  ];

  const newtonTableColumns = [
    {
      label: "№ шага",
      key: "iteration",
    },
    {
      label: "x_k",
      key: "x_k",
    },
    {
      label: "f(x_k)",
      key: "f_x_k",
    },
    {
      label: "f'(x_k)",
      key: "f_prime_x_k",
    },
    {
      label: "x_k+1",
      key: "x_k_plus_one",
    },
    {
      label: "|x_k-x_{k+1}|",
      key: "abs_diff",
    },
  ];

  const handleOpen = () => {
    setIsOpen(true);
  };

  const handleClose = () => {
    setIsOpen(false);
  };

  ChartJS.register(LinearScale, PointElement, LineElement, Tooltip, Legend);

  const handleSubmitString = async (
    event: React.FormEvent<HTMLFormElement>
  ) => {
    event.preventDefault();

    setSolution({
      err: "",
      function_value: 0,
      iterations: 0,
      root: 0,
      steps: [],
    });

    try {
      const { intervalMin, intervalMax, eq_id, estimate, method_id, ...rest } =
        formData;
      const interval: number[] = [intervalMin, intervalMax];

      const response = await fetch(
        "http://127.0.0.1:8000/nonlinear_equations/string",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            eq_id,
            estimate,
            method_id,
            interval,
          }),
        }
      );
      const data = await response.json();

      if (data.error) {
        handleOpen();
        setError(data.error);
        return;
      }
      setSolution(data.result);
      switch (method_id) {
        case 0:
          setHalfdivisonTableRows(data?.result?.steps);
        case 1:
          setSimpleiterationTableRows(data?.result?.steps);
        case 2:
          setNewtonTableRows(data?.result?.steps);
      }
    } catch (error) {
      console.error(error);
      handleOpen();
      setError(`Error while processing: ${error}`);
    }
  };

  const handleSubmitFile = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    setSolution({
      err: "",
      function_value: 0,
      iterations: 0,
      root: 0,
      steps: [],
    });

    try {
      const formMultipartData = new FormData(event.currentTarget);

      const response = await fetch(
        "http://127.0.0.1:8000/nonlinear_equations/file",
        {
          method: "POST",
          body: formMultipartData,
        }
      );
      const data = await response.json();

      if (data.error) {
        handleOpen();
        setError(data.error);
        return;
      }
      console.log(data.result.method_id);
      switch (data.result.method_id) {
        case 0: {
          formData.method_id = 0;
          setHalfdivisonTableRows(data?.result?.steps);
          break;
        }
        case 1: {
          formData.method_id = 1;
          setSimpleiterationTableRows(data?.result?.steps);
          break;
        }
        case 2: {
          formData.method_id = 2;
          setNewtonTableRows(data?.result?.steps);
          break;
        }
      }

      setSolution(data.result);
    } catch (error) {
      console.error(error);
      handleOpen();
      setError(`Error while uploading: ${error}`);
    }
  };

  useEffect(() => {
    switch (formData.method_id) {
      case 0:
        setHalfdivisonTableRows([
          {
            key: 0,
            iteration: 0,
            a: 0,
            b: 0,
            x: 0,
            fa: 0,
            fb: 0,
            fx: 0,
            abs_diff: 0,
            err: "",
          },
        ]);
      case 1:
        setSimpleiterationTableRows([
          {
            key: 0,
            iteration: 0,
            x_k: 0,
            f_x_k: 0,
            x_k_plus_one: 0,
            phi_x_k: 0,
            abs_diff: 0,
            err: "",
          },
        ]);
      case 2:
        setNewtonTableRows([
          {
            key: 0,
            iteration: 0,
            x_k: 0,
            f_x_k: 0,
            f_prime_x_k: 0,
            x_k_plus_one: 0,
            abs_diff: 0,
            err: "",
          },
        ]);
    }
  }, []);

  return (
    <>
      <div className="container mx-auto space-y-12 py-8">
        <div className="border-gray-900/10 border-b-2 pb-12">
          <h1 className="text-3xl font-semibold leading-7 text-gray-900">
            Лабораторная работа
          </h1>
          <p className="mt-1 text-md leading-6 text-gray-600 mb-6">
            «Численное решение нелинейных уравнений и систем нелинейных
            уравнений»
          </p>

          <div className="col-span-full mb-6">
            <label className="block text-md font-medium leading-6 text-gray-900">
              Ручной ввод параметров
            </label>
            <div className="mt-2">
              <form
                onSubmit={handleSubmitString}
                className="grid grid-cols-2 gap-y-4"
              >
                <div className="grid grid-cols-2 gap-y-4 gap-x-4 col-span-2 text-md font-medium leading-6 text-gray-900 items-center">
                  <label className="grid text-md font-medium leading-6 text-gray-900">
                    Equation ID:
                    <input
                      className="block w-50 rounded-md border-0 px-2 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                      type="number"
                      name="eq_id"
                      min={0}
                      max={3}
                      value={formData.eq_id}
                      onChange={handleChange}
                    />
                  </label>
                  <p className="grid col-span-1 text-md font-medium leading-6 text-gray-900">
                    0. 1.62x³ - 8.15x² + 4.39x + 4.29 = 0 <br />
                    1. x³ - x + 4 = 0 <br />
                    2. exp(x) - 5 = 0 <br />
                    3. sin(2*x) + π/4 = 0
                  </p>
                </div>
                <label className="grid col-span-2 text-md font-medium leading-6 text-gray-900">
                  Interval min:
                  <input
                    className="block w-50 rounded-md border-0 px-2 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    type="number"
                    name="intervalMin"
                    value={formData.intervalMin}
                    onChange={handleChange}
                  />
                </label>
                <label className="grid col-span-2 text-md font-medium leading-6 text-gray-900">
                  Interval max:
                  <input
                    className="block w-50 rounded-md border-0 px-2 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    type="number"
                    name="intervalMax"
                    value={formData.intervalMax}
                    onChange={handleChange}
                  />
                </label>
                <label className="grid col-span-2 text-md font-medium leading-6 text-gray-900">
                  Estimate:
                  <input
                    className="block w-50 rounded-md border-0 px-2 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    type="number"
                    name="estimate"
                    value={formData.estimate}
                    onChange={handleChange}
                  />
                </label>
                <label className="grid col-span-2 text-md font-medium leading-6 text-gray-900">
                  Method ID:
                  <input
                    className="block w-50 rounded-md border-0 px-2 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    type="number"
                    name="method_id"
                    min={0}
                    max={2}
                    value={formData.method_id}
                    onChange={handleChange}
                  />
                </label>
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
              Ручной ввод параметров
            </label>
            <div className="mt-2">
              <form onSubmit={handleSubmitFile} encType="multipart/form-data">
                <input
                  name="file"
                  type="file"
                  className="block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
                <div className="mt-2 flex items-center justify-end gap-x-6">
                  <button
                    type="submit"
                    className="rounded-md bg-indigo-600 px-3 py-2 text-md font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                  >
                    Рассчитать
                  </button>
                </div>
              </form>
            </div>
          </div>

          <div className="border-t border-gray-900/10 pb-12">
            <h1 className="text-2xl font-semibold leading-7 text-gray-900 mt-5">
              Решение
            </h1>
            {solution && solution.err && (
              <div
                className="mt-10 grid-cols-2 gap-x-6 gap-y-8 justify-items-center justify-center bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-4"
                role="alert"
              >
                <p className="font-bold">Внимание!</p>
                <p>{solution.err}</p>
              </div>
            )}
            <div className="mt-10 grid grid-cols-2 gap-x-6 gap-y-8 justify-items-center justify-center">
              <div className="col-span-2 w-full">
                <label className="block text-md font-medium leading-6 text-gray-900">
                  Количество итераций
                </label>
                <div className="py-2">
                  <input
                    type="text"
                    name="iter"
                    id="iter"
                    value={solution ? solution.iterations : "None"}
                    disabled
                    className="px-2 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                  />
                </div>
              </div>
              <div className="col-span-1 w-full">
                <label className="block text-md font-medium leading-6 text-gray-900">
                  x
                </label>
                <div className="py-2">
                  <div className="col-span-1 w-fit py-1">
                    <input
                      type="text"
                      name="iter"
                      id="iter"
                      value={solution ? solution.root : "None"}
                      disabled
                      className="px-2 block w-50 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    />
                  </div>
                </div>
              </div>
              <div className="col-span-1 w-full">
                <label className="block text-md font-medium leading-6 text-gray-900">
                  f(x)
                </label>
                <div className="py-2">
                  <div className="col-span-1 w-fit py-1">
                    <input
                      type="text"
                      name="iter"
                      id="iter"
                      value={solution ? solution.function_value : "None"}
                      disabled
                      className="px-2 block w-50 rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    />
                  </div>
                </div>
              </div>

              {formData.method_id == 0 && (
                <div className="col-span-2 w-full">
                  <label className="block text-md font-medium leading-6 text-gray-900">
                    Уточнение корня уравнения методом половинного деления
                  </label>
                  <div className="py-2">
                    <Table aria-label="Example table with dynamic content">
                      <TableHeader columns={halfdivisonTableColumns}>
                        {(column) => (
                          <TableColumn key={column.key}>
                            {column.label}
                          </TableColumn>
                        )}
                      </TableHeader>
                      <TableBody items={halfdivisonTableRows}>
                        {(item: any) => (
                          <TableRow key={item.key}>
                            {(columnKey) => (
                              <TableCell>
                                {getKeyValue(item, columnKey)}
                              </TableCell>
                            )}
                          </TableRow>
                        )}
                      </TableBody>
                    </Table>
                  </div>
                </div>
              )}

              {formData.method_id == 1 && (
                <div className="col-span-2 w-full">
                  <label className="block text-md font-medium leading-6 text-gray-900">
                    Уточнение корня уравнения методом простой итерации
                  </label>
                  <div className="py-2">
                    <Table aria-label="Example table with dynamic content">
                      <TableHeader columns={simpleiterationTableColumns}>
                        {(column) => (
                          <TableColumn key={column.key}>
                            {column.label}
                          </TableColumn>
                        )}
                      </TableHeader>
                      <TableBody items={simpleiterationTableRows}>
                        {(item: any) => (
                          <TableRow key={item.key}>
                            {(columnKey) => (
                              <TableCell>
                                {getKeyValue(item, columnKey)}
                              </TableCell>
                            )}
                          </TableRow>
                        )}
                      </TableBody>
                    </Table>
                  </div>
                </div>
              )}

              {formData.method_id == 2 && (
                <div className="col-span-2 w-full">
                  <label className="block text-md font-medium leading-6 text-gray-900">
                    Уточнение корня уравнения методом Ньютона
                  </label>
                  <div className="py-2">
                    <Table aria-label="Example table with dynamic content">
                      <TableHeader columns={newtonTableColumns}>
                        {(column) => (
                          <TableColumn key={column.key}>
                            {column.label}
                          </TableColumn>
                        )}
                      </TableHeader>
                      <TableBody items={newtonTableRows}>
                        {(item: any) => (
                          <TableRow key={item.key}>
                            {(columnKey) => (
                              <TableCell>
                                {getKeyValue(item, columnKey)}
                              </TableCell>
                            )}
                          </TableRow>
                        )}
                      </TableBody>
                    </Table>
                  </div>
                </div>
              )}

              <div className="col-span-2 w-full">
                <Scatter options={options} data={data} />
              </div>
            </div>
          </div>

          <div id="gd" className="sm:col-span-6 col-span-1"></div>
        </div>
      </div>
    </>
  );
};

export default LinearEquationPage;
