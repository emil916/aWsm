use llvm::Compile;
use llvm::Context as LLVMCtx;
use llvm::Function;
use llvm::FunctionType;
use llvm::Module as LLVMModule;
use llvm::PointerType;
use llvm::Sub;

use super::ModuleCtx;

// Backing functions for wasm operations
pub const INITIALIZE_REGION_STUB: &str = "initialize_region";

pub const GET_MEMORY_PTR: &str = "get_memory_ptr";

pub const TABLE_ADD: &str = "add_function_to_table";
pub const TABLE_FETCH: &str = "get_function_from_table";

pub const I32_ROTL: &str = "rotl_u32";
pub const I32_ROTR: &str = "rotr_u32";

pub const I64_ROTL: &str = "rotl_u64";
pub const I64_ROTR: &str = "rotr_u64";

pub const I32_TRUNC_F32: &str = "i32_trunc_f32";
pub const U32_TRUNC_F32: &str = "u32_trunc_f32";
pub const I32_TRUNC_F64: &str = "i32_trunc_f64";
pub const U32_TRUNC_F64: &str = "u32_trunc_f64";

pub const I64_TRUNC_F32: &str = "i64_trunc_f32";
pub const U64_TRUNC_F32: &str = "u64_trunc_f32";
pub const I64_TRUNC_F64: &str = "i64_trunc_f64";
pub const U64_TRUNC_F64: &str = "u64_trunc_f64";

pub const F32_TRUNC_F32: &str = "f32_trunc_f32";
pub const F64_TRUNC_F64: &str = "f64_trunc_f64";

// Intrinsic llvm functions
pub const I32_CTPOP: &str = "llvm.ctpop.i32";
pub const I64_CTPOP: &str = "llvm.ctpop.i64";

pub const I32_CLZ: &str = "llvm.ctlz.i32";
pub const I64_CLZ: &str = "llvm.ctlz.i64";

pub const I32_CTZ: &str = "llvm.ctlz.i32";
pub const I64_CTZ: &str = "llvm.ctlz.i64";

pub const F32_FABS: &str = "llvm.fabs.f32";
pub const F64_FABS: &str = "llvm.fabs.f64";

pub const F32_SQRT: &str = "llvm.sqrt.f32";
pub const F64_SQRT: &str = "llvm.sqrt.f64";

pub const TRAP: &str = "llvm.trap";

// TODO: Rewrite this using macros, because this is just gross
pub fn insert_runtime_stubs(ctx: &LLVMCtx, m: &LLVMModule) {
    // Initialize region stub, which is a helper function to setup memory
    let initialize_region_type = FunctionType::new(
        <()>::get_type(ctx),
        &[
            <u32>::get_type(ctx),
            <u32>::get_type(ctx),
            PointerType::new(<u8>::get_type(ctx)),
        ],
    );
    m.add_function(INITIALIZE_REGION_STUB, initialize_region_type.to_super());

    // Table interaction function stubs
    let table_add_type = FunctionType::new(
        <()>::get_type(ctx),
        &[
            <u32>::get_type(ctx),
            <u32>::get_type(ctx),
            PointerType::new(<u8>::get_type(ctx)),
        ],
    );
    m.add_function(TABLE_ADD, table_add_type.to_super());

    let table_get_type = FunctionType::new(
        PointerType::new(<u8>::get_type(ctx)),
        &[<u32>::get_type(ctx), <u32>::get_type(ctx)],
    );
    m.add_function(TABLE_FETCH, table_get_type.to_super());

    // Rotate left/right types
    let u32_rot_type = FunctionType::new(
        <u32>::get_type(ctx),
        &[<u32>::get_type(ctx), <u32>::get_type(ctx)],
    );
    m.add_function(I32_ROTL, u32_rot_type.to_super());
    m.add_function(I32_ROTR, u32_rot_type.to_super());

    let i64_rot_type = FunctionType::new(
        <u64>::get_type(ctx),
        &[<u64>::get_type(ctx), <u64>::get_type(ctx)],
    );
    m.add_function(I64_ROTL, i64_rot_type.to_super());
    m.add_function(I64_ROTR, i64_rot_type.to_super());

    // Integer to floating point conversions
    let i32_trunc_f32_type = FunctionType::new(<i32>::get_type(ctx), &[<f32>::get_type(ctx)]);
    m.add_function(I32_TRUNC_F32, i32_trunc_f32_type.to_super());
    m.add_function(U32_TRUNC_F32, i32_trunc_f32_type.to_super());

    let i32_trunc_f64_type = FunctionType::new(<i32>::get_type(ctx), &[<f64>::get_type(ctx)]);
    m.add_function(I32_TRUNC_F64, i32_trunc_f64_type.to_super());
    m.add_function(U32_TRUNC_F64, i32_trunc_f64_type.to_super());

    let i64_trunc_f32_type = FunctionType::new(<i64>::get_type(ctx), &[<f32>::get_type(ctx)]);
    m.add_function(I64_TRUNC_F32, i64_trunc_f32_type.to_super());
    m.add_function(U64_TRUNC_F32, i64_trunc_f32_type.to_super());

    let i64_trunc_f64_type = FunctionType::new(<i64>::get_type(ctx), &[<f64>::get_type(ctx)]);
    m.add_function(I64_TRUNC_F64, i64_trunc_f64_type.to_super());
    m.add_function(U64_TRUNC_F64, i64_trunc_f64_type.to_super());

    let f32_trunc_f32_type = FunctionType::new(<f32>::get_type(ctx), &[<f32>::get_type(ctx)]);
    m.add_function(F32_TRUNC_F32, f32_trunc_f32_type.to_super());

    let f64_trunc_f64_type = FunctionType::new(<f64>::get_type(ctx), &[<f64>::get_type(ctx)]);
    m.add_function(F64_TRUNC_F64, f64_trunc_f64_type.to_super());

    // Memory offset function
    m.add_function(
        GET_MEMORY_PTR,
        FunctionType::new(
            PointerType::new(<i8>::get_type(ctx)),
            &[<u32>::get_type(ctx), <u32>::get_type(ctx)],
        ).to_super(),
    );

    // LLVM intrinsics
    m.add_function(
        I32_CLZ,
        FunctionType::new(
            <i32>::get_type(ctx),
            &[<i32>::get_type(ctx), <bool>::get_type(ctx)],
        ).to_super(),
    );
    m.add_function(
        I64_CLZ,
        FunctionType::new(
            <i64>::get_type(ctx),
            &[<i64>::get_type(ctx), <bool>::get_type(ctx)],
        ).to_super(),
    );
    m.add_function(
        I32_CTZ,
        FunctionType::new(
            <i32>::get_type(ctx),
            &[<i32>::get_type(ctx), <bool>::get_type(ctx)],
        ).to_super(),
    );
    m.add_function(
        I64_CTZ,
        FunctionType::new(
            <i64>::get_type(ctx),
            &[<i64>::get_type(ctx), <bool>::get_type(ctx)],
        ).to_super(),
    );
    m.add_function(
        I32_CTPOP,
        FunctionType::new(<i32>::get_type(ctx), &[<i32>::get_type(ctx)]).to_super(),
    );
    m.add_function(
        I64_CTPOP,
        FunctionType::new(<i64>::get_type(ctx), &[<i64>::get_type(ctx)]).to_super(),
    );
    m.add_function(
        F32_FABS,
        FunctionType::new(<f32>::get_type(ctx), &[<f32>::get_type(ctx)]).to_super(),
    );
    m.add_function(
        F64_FABS,
        FunctionType::new(<f64>::get_type(ctx), &[<f64>::get_type(ctx)]).to_super(),
    );
    m.add_function(
        F32_SQRT,
        FunctionType::new(<f32>::get_type(ctx), &[<f32>::get_type(ctx)]).to_super(),
    );
    m.add_function(
        F64_SQRT,
        FunctionType::new(<f64>::get_type(ctx), &[<f64>::get_type(ctx)]).to_super(),
    );
    m.add_function(TRAP, FunctionType::new(<()>::get_type(ctx), &[]).to_super());
}

pub fn get_stub_function<'a>(m_ctx: &'a ModuleCtx, name: &str) -> &'a Function {
    m_ctx.llvm_module.get_function(name).unwrap()
}