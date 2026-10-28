In Nushell, scripts handle arguments and flags through a main function definition. To access a flag, you define it in the parameters block of main, and Nushell automatically binds it to a local variable inside the script.

### **Step-by-Step Example**

Create a script file (e.g., greet.nu):

```nu
Code snippet  
\# greet.nu  
def main \[  
    name: string,                \# Positional argument  
    \--age: int,                  \# Flag that takes a value  
    \--verbose(-v)                \# Boolean switch flag  
\] {  
    print $"Hello,($name)\!"

    \# Access the flag like a regular variable  
    if \$verbose {  
        print \$"Checking details..."  
    }

    if (\$age \!= null) {  
        print $"Youare($age) years old."  
    }  
}
```

### **Running the Script**

You can pass flags using either longhand or shorthand syntax when executing the script with nu:

Bash  
nu greet.nu Alice \--age 30 \--verbose

### **Key Rules for Flags in Nushell**

* **Dashes to Underscores:** If a flag uses a hyphen (e.g., \--user-dir), Nushell converts it to an underscore for the variable name inside the script (e.g., \$user\_dir).  
* **Boolean Switches:** Flags defined without a type (like \--verbose or \--verbose(-v)) act as boolean switches (true if passed, false if omitted).  
* **Optional Flags:** Named flags are optional by default. If they aren't provided when running the script, their value inside the script will be null (or false for boolean switches).