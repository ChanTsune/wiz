; ModuleID = 'main'
source_filename = "main"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"

@"Hello world" = private unnamed_addr constant [12 x i8] c"Hello world\00", align 1

declare i32 @puts(i8*)

define void @main() {
entry:
  %f_call = call i32 @puts(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @"Hello world", i32 0, i32 0))
  ret void
}
