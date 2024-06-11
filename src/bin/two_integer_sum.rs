
pub mod solution {

    pub fn two_integere_sum(num_list:&[i32], target:i32) -> [usize; 2]
    {
        let mut i: usize = 0;
        let mut j :usize = num_list.len() - 1;
        while  i < j {
            let current_sum = num_list[i] + num_list[j];
            if current_sum == target{
                break;
            }
            else  if current_sum < target{
                i+=1;
            }
            else {
                j-=1;
            }
        }
        return [i + 1, j + 1];
    }
}

fn main()
{
    let num_list : Vec<i32> = vec![1,2,3,4];
    println!("{:?}", solution::two_integere_sum(&num_list, 3));
}