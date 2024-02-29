import {
  Navbar,
  NavbarBrand,
  NavbarContent,
  NavbarItem,
  Link,
  Button,
} from "@nextui-org/react";

export default function Header() {
  return (
    <Navbar isBordered>
      <NavbarBrand>
        {/* <AcmeLogo /> */}
        <p className="font-bold text-inherit"><Link href="/">Skuf Prod.</Link></p>
      </NavbarBrand>
      <NavbarContent className="hidden sm:flex gap-4" justify="center">
        <NavbarItem>
          <Link color="foreground" href="/lab1">
            Лаб. 1
          </Link>
        </NavbarItem>
        <NavbarItem>
          <Link color="foreground" href="/lab2">
            Лаб. 2
          </Link>
        </NavbarItem>
        <NavbarItem>
          <Link color="foreground" href="/lab3">
            Лаб. 3
          </Link>
        </NavbarItem>
        <NavbarItem>
          <Link color="foreground" href="/lab4">
            Лаб. 4
          </Link>
        </NavbarItem>
        <NavbarItem>
          <Link color="foreground" href="/lab5">
            Лаб. 5
          </Link>
        </NavbarItem>
        <NavbarItem>
          <Link color="foreground" href="/lab6">
            Лаб. 6
          </Link>
        </NavbarItem>
      </NavbarContent>
    </Navbar>
  );
}
