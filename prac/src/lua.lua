--------------------------------------------------------------------------------
-- FAKE DIAGNOSTIC TEST (works with :luafile lua.lua)
--------------------------------------------------------------------------------

local bufnr = vim.api.nvim_get_current_buf()
local ns = vim.api.nvim_create_namespace("fake_diags")

-- Make sure buffer has enough lines
vim.api.nvim_buf_set_lines(bufnr, 0, -1, false, {
    "Line 1: Nothing here",
    "Line 2: Nothing here",
    "Line 3: Nothing here",
    "Line 4: Nothing here",
    "Line 5: Nothing here",
    "Line 6: Nothing here",
    "Line 7: Nothing here",
    "Line 8: Nothing here",
    "Line 9: Nothing here",
    "Line 10: THIS LINE HAS 4 FAKE DIAGNOSTICS - Move cursor here",
    "Line 11: Nothing here",
    "Line 12: Nothing here",
})

local fake_diags = {
    {
        lnum = 9,
        col = 0,
        severity = vim.diagnostic.severity.ERROR,
        message = string.rep("EError diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. rror diagnostic example. ", 20),
        source = "rust-analyzer",
    },
    {
        lnum = 9,
        col = 10,
        severity = vim.diagnostic.severity.ERROR,
        message = string.rep("SecoError diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. nd error diagnostic. ", 20),
        source = "rust-analyzer",
    },
    {
        lnum = 9,
        col = 20,
        severity = vim.diagnostic.severity.WARN,
        message = string.rep("WarninError diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. g diagnostic example. ", 20),
        source = "clippy",
    },
    {
        lnum = 9,
        col = 30,
        severity = vim.diagnostic.severity.HINT,
        message = string.rep("HinError Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. Error diagnostic example. diagnostic example. Error diagnostic example. t diagnostic example. ", 20),
        source = "rust-analyzer",
    },
}

vim.diagnostic.set(ns, bufnr, fake_diags, {})

print("Fake diagnostics loaded!")
print("Move cursor to line 10")
